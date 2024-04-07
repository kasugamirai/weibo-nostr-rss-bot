mod uid;
use nostr_sdk::async_utility::futures_util::TryFutureExt;
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

const USER_NAME: [&str; 2] = ["23", "2134"];
const DSN: &str = "123";
const BASE_URL: &str = "https://weibrss.oneoo.info";


pub struct App {
    db: DbConnection,
}

impl App {
    pub fn new(dsn: &str) -> Self {
        let db = DbConnection::new(dsn).unwrap_or_else(|e| {
            panic!("Failed to create database connection: {}", e);
        });
        App { db }
    }

    async fn get_uid(&self,name:&str) -> String {
        let existed = self.db.uid_exists(name).await.unwrap();
        if !existed {
            let WeiboUid = WeiboUid::new(BASE_URL);

        }
        
        return "ok";
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
