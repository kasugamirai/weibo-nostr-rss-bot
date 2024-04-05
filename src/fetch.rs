use reqwest::Error;
use rss::Channel;

pub struct Fetcher<'a> {
    uid: &'a str,
}

pub struct UserInfo {
    pub title: String,
    pub image_url: String,
}

pub struct Message {
    pub title: String,
    pub link: String,
    pub description: String,
}

impl<'a> Fetcher<'a> {
    pub fn new(uid: &'a str) -> Self {
        Fetcher { uid }
    }

    pub async fn fetch_user_info(&self) -> Result<UserInfo, Error> {
        let url = format!("https://rsshub.app/weibo/user/{}", self.uid);
        let response = reqwest::get(&url).await?.bytes().await?;
        let channel = Channel::read_from(&response[..]).unwrap();

        let title = channel.title().to_string();
        let image_url = channel
            .image()
            .map(|image| image.url().to_string())
            .unwrap_or_else(|| "No image URL".to_string());
        Ok(UserInfo { title, image_url })
    }

    pub async fn fetch_messages(&self) -> Result<Vec<Message>, Error> {
        let url = format!("https://rsshub.app/weibo/user/{}", self.uid);
        let response = reqwest::get(&url).await?.bytes().await?;
        let channel = Channel::read_from(&response[..]).unwrap();

        let messages = channel
            .items()
            .into_iter()
            .map(|item| Message {
                title: item.title().unwrap_or("No title").to_string(),
                link: item.link().unwrap_or("No link").to_string(),
                description: item.description().unwrap_or("No description").to_string(),
            })
            .collect();
        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_user_info() {
        let fetcher = Fetcher::new("1883568433");
        let result = fetcher.fetch_user_info().await;

        match result {
            Ok(user_info) => {
                assert!(!user_info.title.is_empty());
                assert!(!user_info.image_url.is_empty());
            }
            Err(e) => {
                panic!("Failed to fetch user info: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_messages() {
        let fetcher = Fetcher::new("1883568433");
        let result = fetcher.fetch_messages().await;

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