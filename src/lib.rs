mod uid;
use nostr_sdk::async_utility::futures_util::TryFutureExt;
use nostr_sdk::secp256k1::SecretKey;
use anyhow::{Result, anyhow};
use nostr_sdk::Keys;
use nostr_sdk::ToBech32;
pub use uid::WeiboUid;
mod conf;
mod rss;
mod nostr;
mod msg;
pub use nostr::NotePublisher;
use nostr_db::DbConnection;
pub use msg::Message;
pub use msg::UserInfo;
pub use rss::Rss;
use std::string::String;

const USER_NAME: [&str; 2] = ["23", "2134"];
const DSN: &str = "123";
const BASE_URL: &str = "https://weibrss.oneoo.info";


pub struct App {
    db: DbConnection,
}

#[derive(Clone)]
pub struct MyKey {
    public_key: String,
    secret_key: String,
}



impl App {

    pub fn new_key(&self) -> Result<MyKey> {
        let my_keys: Keys = Keys::generate();
        let pk = my_keys.public_key().to_bech32()?;
        let prk = my_keys.secret_key()?.to_bech32()?;
        let ret = MyKey{
            public_key:pk,
            secret_key:prk,
        };
        Ok(ret)
    }

    pub fn new(dsn: &str) -> Self {
        let db = DbConnection::new(dsn).unwrap_or_else(|e| {
            panic!("Failed to create database connection: {}", e);
        });
        App { db }
    }

    async fn get_uid(& mut self,name:&str) -> Result<String> {
        let existed = self.db.uid_exists(name).await.unwrap();
        let uid;
        if !existed {
            let weibo_uid = WeiboUid::new(BASE_URL);
            uid = weibo_uid.get_weibo_uid(name).await.unwrap();
            let uidi32:i32 = uid.parse()?;
            let rss  = Rss::new(&uid);
            let uifo = rss.fetch_user_info().await?;
            let key  =   self.new_key()?;
            self.db.add_user(name, &uifo.image_url, &key.public_key, &key.secret_key, uidi32);
        } else {
             uid = self.db.query_u_id(name).await.unwrap().unwrap();
        }
        Ok(uid)
    }

    async fn test() {
        todo!();
        /*
        let rss = Rss::new();
        let url = "https://rsshub.app/weibo/user/1883568433";
        let result = fetcher.fetch_user_info(url).await;

        match result {
            Ok((title, image_url)) => {
                let message_url = "https://rsshub.app/weibo/user/1883568433";
                let messages = fetcher.fetch_messages(message_url).await.unwrap();
                for message in messages {
                    db.save_message(&message).await.unwrap();
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        */
    }
}
