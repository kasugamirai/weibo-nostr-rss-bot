use nostr_sdk::bitcoin::network::message;
use reqwest::Error;
use weibo_nostr::App;
//use weibo_nostr::fetch_and_print_rss;
const DSN: &str = "123";
const NAME: &str = "123";


#[tokio::main]
async fn main() {
    let mut app = App::new(DSN);
    let uid = app.get_uid(NAME).await.unwrap();
    let messages = app.get_contents(&uid).await.unwrap();
    for msg in messages {
        let _ = app.publish(NAME, &msg.description).await.unwrap();
    }

}
