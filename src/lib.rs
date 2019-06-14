//! A crate for creating Twitter bots
//!
//! # Example
//!
//! ```rust
//! use rust_twitter_bot_lib::*;
//!
//! fn main() {
//!   let example_bot = TwitterBot::new()
//!     .consumer_key(YOUR_CONSUMER_KEY)
//!     .consumer_secret_key(YOUR_CONSUMER_SECRET_KEY)
//!     .access_token(YOUR_ACCESS_TOKEN)
//!     .secret_access_token(YOUR_SECRET_ACCESS_TOKEN);
//!
//!   let res = example_bot.tweet("🐦 + 🦀 = 💙 #myfirstTweet").unwrap();
//!
//!   println!("{:?}", res);
//! }
//! ```
extern crate oauthcli;
extern crate reqwest;
extern crate serde;
extern crate url;

mod tweet_structure;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    path::Path,
};
pub use tweet_structure::{Tweet, User};

use reqwest::multipart;
use serde::{Deserialize, Serialize};

/// The main struct provided by this crate. See crate documentation for more
/// information.
#[derive(Default, Serialize, Deserialize)]
pub struct TwitterBot {
    consumer_key: Option<String>,
    consumer_secret_key: Option<String>,
    access_token: Option<String>,
    secret_access_token: Option<String>,
}

impl TwitterBot {
    /// Creates a new Empty `TwitterBot`
    pub fn new() -> Self {
        TwitterBot::default()
    }

    /// Add your `consumer_key`<br/>  
    /// Get it in your [Twitter App Dashboard](https://developer.twitter.com/en/apps/)
    pub fn consumer_key(self, consumer_key: &str) -> Self {
        Self {
            consumer_key: Some(consumer_key.to_owned()),
            ..self
        }
    }

    /// Add your `consumer_secret_key`<br/>  
    /// Get it in your [Twitter App Dashboard](https://developer.twitter.com/en/apps/)
    pub fn consumer_secret_key(self, consumer_secret_key: &str) -> Self {
        Self {
            consumer_secret_key: Some(consumer_secret_key.to_owned()),
            ..self
        }
    }

    /// Add your `access_token`<br/>  
    /// Get it in your [Twitter App Dashboard](https://developer.twitter.com/en/apps/)
    pub fn access_token(self, access_token: &str) -> Self {
        Self {
            access_token: Some(access_token.to_owned()),
            ..self
        }
    }

    /// Add your `secret_access_token`<br/>  
    /// Get it in your [Twitter App Dashboard](https://developer.twitter.com/en/apps/)
    pub fn secret_access_token(self, secret_access_token: &str) -> Self {
        Self {
            secret_access_token: Some(secret_access_token.to_owned()),
            ..self
        }
    }

    fn is_connected(&self) -> Option<Box<Error>> {
        if self.consumer_key.is_none() {
            return Some(TwitterBotError::new("consumer_key missing").into());
        } else if self.consumer_secret_key.is_none() {
            return Some(TwitterBotError::new("consumer_secret_key missing").into());
        } else if self.access_token.is_none() {
            return Some(TwitterBotError::new("access_token missing").into());
        } else if self.secret_access_token.is_none() {
            return Some(TwitterBotError::new("secret_access_token missing").into());
        }
        return None;
    }

    fn send_request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        url: url::Url,
        method: &str,
    ) -> Result<T, Box<Error>> {
        if let Some(err) = self.is_connected() {
            return Err(err);
        }

        let header = oauthcli::OAuthAuthorizationHeaderBuilder::new(
            method,
            &url,
            self.consumer_key.as_ref().unwrap(),
            self.consumer_secret_key.as_ref().unwrap(),
            oauthcli::SignatureMethod::HmacSha1,
        )
        .token(
            self.access_token.as_ref().unwrap(),
            self.secret_access_token.as_ref().unwrap(),
        )
        .finish_for_twitter();

        let client = reqwest::Client::new();
        let mut response = if method == "POST" {
            client
                .post(&url.to_string())
                .header("Authorization", header.to_string())
                .send()?
        } else if method == "GET" {
            client
                .get(&url.to_string())
                .header("Authorization", header.to_string())
                .send()?
        } else {
            panic!("Invalid method");
        };

        if response.status() == 200 {
            return Ok(response.json()?);
        } else {
            let err: tweet_structure::TwitterError = response.json()?;
            return Err(TwitterBotError::new(&err.message()).into());
        }
    }

    /// Create a media from a file<br/>
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn upload_file(&self, path: &Path) -> Result<u64, Box<Error>> {
        if let Some(err) = self.is_connected() {
            return Err(err);
        }

        let header = oauthcli::OAuthAuthorizationHeaderBuilder::new(
            "POST",
            &url::Url::parse("https://upload.twitter.com/1.1/media/upload.json")?,
            self.consumer_key.as_ref().unwrap(),
            self.consumer_secret_key.as_ref().unwrap(),
            oauthcli::SignatureMethod::HmacSha1,
        )
        .token(
            self.access_token.as_ref().unwrap(),
            self.secret_access_token.as_ref().unwrap(),
        )
        .finish_for_twitter();

        let form = multipart::Form::new().part("media", multipart::Part::file(path)?);

        let client = reqwest::Client::new();

        let mut response = client
            .post("https://upload.twitter.com/1.1/media/upload.json")
            .header("Authorization", header.to_string())
            .multipart(form)
            .send()?;

        if response.status() == 200 {
            let res: tweet_structure::Media = response.json()?;
            return Ok(res.media_id);
        } else {
            let err: tweet_structure::TwitterError = response.json()?;
            return Err(TwitterBotError::new(&err.message()).into());
        }
    }

    /// Tweet `content`<br/>  
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn tweet(
        &self,
        content: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<tweet_structure::Tweet, Box<Error>> {
        let mut request = url::Url::parse("https://api.twitter.com/1.1/statuses/update.json")?;

        {
            let mut query_pairs = request.query_pairs_mut();
            query_pairs.append_pair("status", content);
            if let Some(pairs) = params {
                for (key, value) in pairs.iter() {
                    query_pairs.append_pair(key, value);
                }
            }
        }

        let request = url::Url::parse(&request.to_string().replace("+", "%20"))?;

        Ok(self.send_request(request, "POST")?)
    }

    /// Favorite a tweet<br/>  
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn favorite(
        &self,
        tweet_id: i64,
    ) -> Result<tweet_structure::Tweet, Box<Error>> {
        let mut request = url::Url::parse("https://api.twitter.com/1.1/favorites/create.json")?;
        {
            let mut query_pairs = request.query_pairs_mut();
            query_pairs.append_pair("id", &tweet_id.to_string());
        }
        
        Ok(self.send_request(request, "POST")?)
    }

    /// Unfavorite a tweet<br/>  
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn unfavorite(
        &self,
        tweet_id: i64,
    ) -> Result<tweet_structure::Tweet, Box<Error>> {
        let mut request = url::Url::parse("https://api.twitter.com/1.1/favorites/destroy.json")?;
        {
            let mut query_pairs = request.query_pairs_mut();
            query_pairs.append_pair("id", &tweet_id.to_string());
        }
        
        Ok(self.send_request(request, "POST")?)
    }

    /// Retweet a tweet<br/>  
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn retweet(
        &self,
        tweet_id: i64,
    ) -> Result<tweet_structure::Tweet, Box<Error>> {
        let request = url::Url::parse(&format!("https://api.twitter.com/1.1/statuses/retweet/{}.json", tweet_id))?;
        
        Ok(self.send_request(request, "POST")?)
    }

    /// Retweet a tweet<br/>  
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn unretweet(
        &self,
        tweet_id: i64,
    ) -> Result<tweet_structure::Tweet, Box<Error>> {
        let request = url::Url::parse(&format!("https://api.twitter.com/1.1/statuses/unretweet/{}.json", tweet_id))?;
        
        Ok(self.send_request(request, "POST")?)
    }

    /// Get tweet with id = `tweet_id`<br/>  
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn get_tweet(&self, tweet_id: &str) -> Result<tweet_structure::Tweet, Box<Error>> {
        let mut request = url::Url::parse("https://api.twitter.com/1.1/statuses/show.json")?;
        request.query_pairs_mut().append_pair("id", tweet_id);

        Ok(self.send_request(request, "GET")?)
    }

    /// Get tweet that satisfy `query`<br/>  
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn get_tweets_query(
        &self,
        query: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<Vec<tweet_structure::Tweet>, Box<Error>> {
        let mut request = url::Url::parse("https://api.twitter.com/1.1/search/tweets.json")?;

        {
            let mut query_pairs = request.query_pairs_mut();
            query_pairs.append_pair("q", query);
            query_pairs.append_pair("count", "100");
            if let Some(pairs) = params {
                for (key, value) in pairs.iter() {
                    query_pairs.append_pair(key, value);
                }
            }
        }

        let request = url::Url::parse(&request.to_string().replace("+", "%20"))?;

        let response: tweet_structure::SearchResponse = self.send_request(request, "GET")?;

        Ok(response.statuses)
    }
}

#[derive(Debug)]
struct TwitterBotError(String);

impl Display for TwitterBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TwitterBotError {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl TwitterBotError {
    fn new(description: &str) -> Self {
        Self(description.to_owned())
    }
}
