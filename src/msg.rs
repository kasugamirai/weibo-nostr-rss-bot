use nostr_db::DbConnection;
use std::result::Result;

#[derive(Debug)]
pub struct UserInfo {
    pub title: String,
    pub image_url: String,
}

#[derive(Debug)]
pub struct Message {
    pub title: String,
    pub link: String,
    pub description: String,
}

pub struct Incoming{
    user_info: UserInfo,
    msg: Message,
    db: DbConnection,
}

impl UserInfo {
    pub fn new(t:String, i:String) -> Self {
        return
            UserInfo{
                title:t,
                image_url:i,
            }
        
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
