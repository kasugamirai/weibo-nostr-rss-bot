use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::contents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contents {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub published: bool,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::contents)]
pub struct NewContents {
    pub author: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub published: bool,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub publickey: String,
    pub privatekey: String,
    pub u_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUsers {
    pub username: String,
    pub avatar: Option<String>,
    pub publickey: String,
    pub privatekey: String,
    pub u_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dsn: String,
}
