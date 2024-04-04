use std::fmt;

const BASE_URL: &str = "https://weibrss.oneoo.info";

pub enum Error {
    Reqwest(reqwest::Error),
    HttpError(reqwest::StatusCode),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Error::HttpError(e) => write!(f, "HTTP error: {}", e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}
impl From<reqwest::StatusCode> for Error {
    fn from(e: reqwest::StatusCode) -> Self {
        Error::HttpError(e)
    }
}

pub struct WeiboUid {
    base_url: String,
}

impl WeiboUid {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get_weibo_uid(&self, domain: &str) -> Result<String, Error> {
        let url = format!("{}/convert?domain={}", self.base_url, domain);
        let response = reqwest::get(&url).await?;

        if response.status().is_success() {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(Error::HttpError(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_weibo_uid() {
        let fetcher = WeiboUid::new(BASE_URL.to_string());
        let result = fetcher.get_weibo_uid("dmmusic").await;

        match result {
            Ok(body) => {
                assert!(body.contains("1883568433"));
            }
            Err(e) => {
                panic!("Failed to get Weibo UID: {}", e);
            }
        }
    }
}
