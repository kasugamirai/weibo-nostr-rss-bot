use reqwest::Error;
use rss::Channel;

pub async fn fetch_user_info(url: &str) -> Result<(String, String), Error> {
    let response = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&response[..]).unwrap();

    let title = channel.title().to_string();
    let image_url = channel
        .image()
        .map(|image| image.url().to_string())
        .unwrap_or("No image URL".to_string());

    Ok((title, image_url))
}

pub async fn fetch_messages(url: &str) -> Result<Vec<(String, String, String)>, Error> {
    let response = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&response[..]).unwrap();

    let messages = channel
        .items()
        .into_iter()
        .map(|item| {
            let title = item.title().unwrap_or("No title").to_string();
            let link = item.link().unwrap_or("No link").to_string();
            let description = item.description().unwrap_or("No description").to_string();

            (title, link, description)
        })
        .collect();

    Ok(messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_user_info() {
        let url = "https://rsshub.app/weibo/user/1883568433";
        let result = fetch_user_info(url).await;

        match result {
            Ok((title, image_url)) => {
                assert!(!title.is_empty());
                assert!(!image_url.is_empty());
            }
            Err(e) => {
                panic!("Failed to fetch user info: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_messages() {
        let url = "https://rsshub.app/weibo/user/1883568433";
        let result = fetch_messages(url).await;

        match result {
            Ok(messages) => {
                assert!(!messages.is_empty());
            }
            Err(e) => {
                panic!("Failed to fetch messages: {}", e);
            }
        }
    }
}
