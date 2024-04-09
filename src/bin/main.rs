use nostr_sdk::bitcoin::network::message;
use nostr_sdk::nips::nip26::Error::ConditionsParseInvalidCondition;
use reqwest::Error;
use std::fs::File;
use std::io::BufReader;
use weibo_nostr::load_conf;
use weibo_nostr::App;
use weibo_nostr::Config;

#[tokio::main]
async fn main() {
    env_logger::init();
    let conf = load_conf("config/conf.yaml").unwrap();
    let dsn = conf.postgres.dsn;
    let names = conf.user.name;

    let mut app = App::new(&dsn);
    for name in names {
        let uid = match app.get_uid(&name).await {
            Ok(uid) => uid,
            Err(e) => {
                eprintln!("Invalid user ID: {}, {}", name, e);
                continue;
            }
        };
        let messages = app
            .get_contents(&uid)
            .await
            .expect("Failed to get messages");
        for msg in messages {
            println!("Publishing message: {} ", msg.title);
            match app.publish(&name, &msg.description).await {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to publish message: {}", e),
            }
        }
    }
}
