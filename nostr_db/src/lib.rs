mod models;
mod schema;

use crate::models::{Contents, NewContents, NewUsers, Users};
use diesel::RunQueryDsl;
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../nostr_db/migrations");

#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    Connection(diesel::ConnectionError),
    IoError(std::io::Error),
    SerdeError(serde_yaml::Error),
}

impl From<diesel::ConnectionError> for Error {
    fn from(err: diesel::ConnectionError) -> Self {
        Error::Connection(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::Diesel(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::SerdeError(err)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Diesel(err) => write!(f, "Diesel error: {}", err),
            Error::Connection(err) => write!(f, "Connection error: {}", err),
            Error::IoError(err) => write!(f, "IO error: {}", err),
            Error::SerdeError(err) => write!(f, "Serde error: {}", err),
        }
    }
}

pub struct DbConnection {
    conn: PgConnection,
}

impl DbConnection {
    pub fn new(dsn: &str) -> Result<DbConnection, Error> {
        let conn = PgConnection::establish(dsn)?;
        Ok(DbConnection { conn })
    }

    pub fn run_migrations(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.conn.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }

    fn load_users(&mut self, name: &str) -> Result<Vec<Users>, Error> {
        use crate::schema::users::dsl::*;
        Ok(users
            .filter(username.eq(name))
            .load::<Users>(&mut self.conn)?)
    }

    pub fn add_avatar(&mut self, name: &str, av: &str) -> Result<(), Error> {
        use crate::schema::users::dsl::*;

        Ok(diesel::update(users.filter(username.eq(name)))
            .set(avatar.eq(av))
            .execute(&mut self.conn)
            .map_err(|err| {
                log::error!("Error adding avatar: {}", err);
                err
            })
            .map(|_| ())?)
    }

    pub async fn query_u_id(&mut self, name: &str) -> Result<Option<i64>, Error> {
        let results = self.load_users(name)?;
        Ok(results.first().map(|user| user.u_id))
    }

    pub async fn query_avatar(&mut self, name: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(name)?;
        Ok(results.first().and_then(|user| user.avatar.clone()))
    }

    pub async fn content_exists(&mut self, lk: &str) -> Result<bool, Error> {
        use crate::schema::contents::dsl::*;
        let results = contents
            .filter(link.eq(lk))
            .load::<Contents>(&mut self.conn)?;
        Ok(results.len() > 0)
    }

    pub async fn uid_exists(&mut self, name: &str) -> Result<bool, Error> {
        let results = self.load_users(name)?;
        Ok(!results.is_empty())
    }

    pub async fn add_user(
        &mut self,
        un: &str,
        av: &str,
        pk: &str,
        prk: &str,
        uid: i64,
    ) -> Result<(), Error> {
        use crate::schema::users::dsl::*;

        let new_user = NewUsers {
            username: un.to_string(),
            avatar: Some(av.to_string()),
            publickey: pk.to_string(),
            privatekey: prk.to_string(),
            u_id: uid,
        };

        Ok(diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut self.conn)
            .map_err(|err| {
                log::error!("Error adding user: {}", err);
                err
            })
            .map(|_| ())?)
    }

    pub async fn query_user_id(&mut self, n: &str) -> Result<Option<i32>, Error> {
        let results = self.load_users(n)?;
        Ok(results.first().map(|user| user.id))
    }

    pub async fn query_user_name(&mut self, uid: i64) -> Result<Option<String>, Error> {
        use crate::schema::users::dsl::*;
        let results = users.filter(u_id.eq(uid)).load::<Users>(&mut self.conn)?;
        Ok(results.first().map(|user| user.username.to_string()))
    }

    pub async fn add_contents(
        &mut self,
        au: &str,
        ti: &str,
        lk: &str,
        de: &str,
        pu: bool,
    ) -> Result<(), Error> {
        use crate::schema::contents::dsl::*;

        let u = self.query_user_id(&au).await?.unwrap();

        let new_content = NewContents {
            author: au.to_string(),
            title: ti.to_string(),
            link: lk.to_string(),
            description: de.to_string(),
            published: pu,
            user_id: u,
        };

        Ok(diesel::insert_into(contents)
            .values(&new_content)
            .execute(&mut self.conn)
            .map_err(|err| {
                log::error!("Error adding video: {}", err);
                err
            })
            .map(|_| ())?)
    }

    pub async fn find_user_private_key(
        &mut self,
        user_name: &str,
    ) -> Result<Option<String>, Error> {
        let results = self.load_users(user_name)?;
        Ok(results.first().map(|user| user.privatekey.to_string()))
    }

    pub async fn find_user_public_key(&mut self, user_name: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(user_name)?;
        Ok(results.first().map(|user| user.publickey.to_string()))
    }
}
