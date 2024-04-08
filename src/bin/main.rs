use std::fs::File;
use std::io::BufReader;
use nostr_sdk::base64::engine::Config;
use nostr_sdk::bitcoin::network::message;
use reqwest::Error;
use weibo_nostr::App;


#[tokio::main]
async fn main() {
    env_logger::init();

    let file = match File::open("./conf/test/config.yaml") {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to open config file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);

    let config: Config = match serde_yaml::from_reader(reader) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to read config: {}", e);
            return;
        }
    };

    let dsn = config.dsn;
    let name = config.name;

    let mut app = App::new(dsn);
    let uid = app.get_uid(NAME).await.unwrap();
    let messages = app.get_contents(&uid).await.unwrap();
    for msg in messages {
        let _ = app.publish(NAME, &msg.description).await.unwrap();
    }
}
