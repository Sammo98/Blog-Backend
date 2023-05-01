use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Payload {
    pub id:String,
    pub blog:Blog
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Blog {
    pub content: String,
    pub metadata: Metadata
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Metadata {
    title:String,
    subtitle:String,
    tags:Vec<String>,
    date:String
}


