use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    id: i64,
    id_str: String,
    text: String,
    truncated: bool,
    in_reply_to_status_id: Option<i64>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<i64>,
    in_reply_to_user_id_str: Option<String>,
    in_reply_to_screen_name: Option<String>,
    user: User,
    is_quote_status: bool,
    retweet_count: i64,
    favorite_count: i64,
    favorited: bool,
    retweeted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i64,
    id_str: String,
    name: String,
    screen_name: String,
    location: Option<String>,
    description: Option<String>,
    url: Option<String>,
    followers_count: i64,
    friends_count: i64,
    listed_count: i64,
    favourites_count: i64,
    statuses_count: i64,
    following: Option<bool>,
    follow_request_sent: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub statuses: Vec<Tweet>,
    search_metadata: SearchMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchMetadata {
    completed_in: f64,
    max_id: i64,
    max_id_str: String,
    query: String,
    refresh_url: String,
    count: i64,
    since_id: i64,
    since_id_str: String,
}

impl Tweet {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn reply_to(&self) -> Option<i64> {
        self.in_reply_to_status_id
    }

    pub fn user(&self) -> &User {
        &self.user
    }
    pub fn content(&self) -> &str {
        &self.text
    }
}

impl User {
    pub fn id(&self) -> &str {
        &self.id_str
    }

    pub fn name(&self) -> &str {
        &self.screen_name
    }
}

#[derive(Serialize, Deserialize)]
pub struct Media {
    pub media_id: u64,
}

#[derive(Serialize, Deserialize)]
struct Errors {
    code: i64,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct TwitterError {
    errors: Vec<Errors>,
}

impl TwitterError {
    pub fn message(&self) -> &str {
        &self.errors[0].message
    }
}

#[derive(Default)]
pub struct QueryOption {
    pub result_type: Option<String>,
    pub since_id: Option<i64>,
    pub max_id: Option<i64>,
}
