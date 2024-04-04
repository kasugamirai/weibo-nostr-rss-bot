mod uid;
pub use uid::WeiboUid;
mod conf;
mod fetch;
mod nostr;
pub use nostr::NotePublisher;
use nostr_db::DbConnection;

pub struct App {
    fetcher: fetch::Fetcher,
    db: DbConnection,
}

impl App {
    pub fn new(fetcher: fetch::Fetcher, db: DbConnection) -> Self {
        Self { fetcher, db }
    }
    async fn run() {
        let fetcher = fetch::Fetcher::new();
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
    }
}
