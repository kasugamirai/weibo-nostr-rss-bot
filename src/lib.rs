mod uid;
use anyhow::{anyhow, Result};
pub use conf::load_conf;
use nostr_sdk::async_utility::futures_util::TryFutureExt;
use nostr_sdk::serde_json;
use nostr_sdk::serde_json::de;
use nostr_sdk::Keys;
use nostr_sdk::SecretKey;
use nostr_sdk::ToBech32;
pub use uid::WeiboUid;
mod conf;
mod msg;
mod nostr;
mod rss;
pub use conf::Config;
use log::error;
pub use msg::Message;
pub use msg::UserInfo;
pub use nostr::NotePublisher;
use nostr_db::DbConnection;
pub use rss::Rss;
use std::str::FromStr;

const BASE_URL: &str = "https://weibrss.oneoo.info";
const CONF_PATH: &str = "config/conf.yaml";

pub struct App {
    db: DbConnection,
}

#[derive(Clone)]
pub struct MyKey {
    public_key: String,
    secret_key: String,
}

impl App {
    fn new_key(&self) -> Result<MyKey> {
        let my_keys: Keys = Keys::generate();
        let pk = my_keys.public_key().to_bech32()?;
        let prk = my_keys.secret_key()?.to_bech32()?;
        let ret = MyKey {
            public_key: pk,
            secret_key: prk,
        };
        Ok(ret)
    }

    pub fn new(dsn: &str) -> Self {
        let db = DbConnection::new(dsn).unwrap_or_else(|e| {
            panic!("Failed to create database connection: {}", e);
        });
        App { db }
    }

    pub async fn get_uid(&mut self, name: &str) -> Result<String> {
        let existed = self.db.uid_exists(name).await?;
        let uid: String;
        if !existed {
            print!("{} does not exist, ", name);
            let weibo_uid = WeiboUid::new(BASE_URL);
            let weibo_uid = weibo_uid.get_weibo_uid(name).await?;
            let parsed: serde_json::Value = serde_json::from_str(&weibo_uid)?;
            uid = parsed["uid"]
                .as_str()
                .ok_or(anyhow::anyhow!("UID not found"))?
                .to_string();
            let uid64: i64 = uid.parse()?;
            let rss = Rss::new(&uid);
            let uifo = rss.fetch_user_info().await?;
            let key = self.new_key()?;
            self.db
                .add_user(
                    name,
                    &uifo.image_url,
                    &key.public_key,
                    &key.secret_key,
                    uid64,
                )
                .await?;
        } else {
            let u = self.db.query_u_id(name).await?;
            uid = u.ok_or(anyhow::anyhow!("User ID not found"))?.to_string();
        }
        Ok(uid)
    }

    pub async fn get_contents(&mut self, uid: &str) -> Result<Vec<Message>> {
        let rss = Rss::new(uid);
        let uid_i64: i64 = uid.parse().expect("Failed to parse uid");
        let uname = self.db.query_user_name(uid_i64).await?.unwrap();
        let msg = rss.fetch_messages().await?;
        let mut ret = Vec::new();
        for m in msg {
            let existed = self.db.content_exists(&m.link).await?;
            if !existed {
                self.db
                    .add_contents(&uname, &m.link, &m.link, &m.description, false)
                    .await
                    .unwrap();
            }
            ret.push(m);
        }
        print!("{} new messages found", ret.len());
        Ok(ret)
    }

    pub async fn publish(&mut self, user_name: &str, message: &str) -> Result<bool> {
        let secret_key = match self.db.find_user_private_key(user_name).await {
            Ok(Some(key)) => key,
            Ok(None) => {
                error!("User private key not found for user: {}", user_name);
                return Err(anyhow::Error::msg("User private key not found"));
            }
            Err(e) => {
                error!("Error finding user private key: {}", e);
                return Err(e.into());
            }
        };

        let avatar = match self.db.query_avatar(user_name).await {
            Ok(Some(avatar)) => avatar,
            Ok(None) => {
                error!("Avatar not found for user: {}", user_name);
                return Err(anyhow::Error::msg("Avatar not found"));
            }
            Err(e) => {
                error!("Error querying avatar: {}", e);
                return Err(e.into());
            }
        };

        let key = match self.convert_key(&secret_key) {
            Ok(key) => key,
            Err(e) => {
                error!("Error converting key: {}", e);
                return Err(e);
            }
        };

        let note_publish = match NotePublisher::new(&key, CONF_PATH).await {
            Ok(publisher) => publisher,
            Err(e) => {
                error!("Error creating NotePublisher: {}", e);
                return Err(e.into());
            }
        };
        note_publish.connect().await;
        if let Err(e) = note_publish.set_metadata(&user_name, &avatar).await {
            log::error!("Failed to set metadata: {}", e);
        }
        if let Err(e) = note_publish
            .publish_text_note(&key, &format!("{}", message))
            .await
        {
            log::error!("Failed to publish text note: {}", e);
        }
        note_publish.disconnect().await;

        Ok(true)
    }

    fn convert_key(&self, secret_key: &str) -> Result<Keys> {
        let sk = SecretKey::from_str(secret_key)?;
        let key = Keys::new(sk);
        Ok(key)
    }
}
