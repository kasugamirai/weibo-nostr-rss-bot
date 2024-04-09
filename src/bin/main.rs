use weibo_nostr::load_conf;
use weibo_nostr::App;

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
            match app.publish(&name, &msg.description).await {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to publish message: {}", e),
            }
        }
    }
}
