use nostr_db;
use std::result::Result;
pub struct UserInfo {
    pub title: String,
    pub image_url: String,
}

pub struct Message {
    pub title: String,
    pub link: String,
    pub description: String,
}

impl UserInfo {
    pub fn new(t:String, i:String) -> Self {
        return
            UserInfo{
                title:t,
                image_url:i,
            }
        
    }
    pub uid_existed(username:String) -> Result<bool, Error> {

    }
}

impl Message {
    pub fn new(t:String, l:String, d: String) -> Self {
        return Message{
            title:t,
            link:l,
            description:d,
        }
    }
}
