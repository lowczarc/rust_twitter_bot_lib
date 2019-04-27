extern crate oauthcli;
extern crate reqwest;
extern crate serde;
extern crate url;

pub mod tweet_structure;

use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Default)]
pub struct TwitterBot<'a> {
    consumer_key: Option<&'a str>,
    consumer_secret_key: Option<&'a str>,
    access_token: Option<&'a str>,
    secret_access_token: Option<&'a str>,
}

impl<'a> TwitterBot<'a> {
    /// Creates a new Empty `TwitterBot`
    pub fn new() -> Self {
        TwitterBot::default()
    }

    /// Add your `consumer_key`<br/>  
    /// Get it in your [Twitter App Dashboard](https://developer.twitter.com/en/apps/)
    pub fn consumer_key(self, consumer_key: &'a str) -> Self {
        Self {
            consumer_key: Some(consumer_key.clone()),
            ..self
        }
    }

    /// Add your `consumer_secret_key`<br/>  
    /// Get it in your [Twitter App Dashboard](https://developer.twitter.com/en/apps/)
    pub fn consumer_secret_key(self, consumer_secret_key: &'a str) -> Self {
        Self {
            consumer_secret_key: Some(consumer_secret_key.clone()),
            ..self
        }
    }

    /// Add your `access_token`<br/>  
    /// Get it in your [Twitter App Dashboard](https://developer.twitter.com/en/apps/)
    pub fn access_token(self, access_token: &'a str) -> Self {
        Self {
            access_token: Some(access_token.clone()),
            ..self
        }
    }

    /// Add your `secret_access_token`<br/>  
    /// Get it in your [Twitter App Dashboard](https://developer.twitter.com/en/apps/)
    pub fn secret_access_token(self, secret_access_token: &'a str) -> Self {
        Self {
            secret_access_token: Some(secret_access_token.clone()),
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
            self.consumer_key.unwrap(),
            self.consumer_secret_key.unwrap(),
            oauthcli::SignatureMethod::HmacSha1,
        )
        .token(
            self.access_token.unwrap(),
            self.secret_access_token.unwrap(),
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

    /// Tweet `content`<br/>  
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn tweet(&self, content: &str) -> Result<tweet_structure::Tweet, Box<Error>> {
        let mut request = url::Url::parse("https://api.twitter.com/1.1/statuses/update.json")?;
        request.query_pairs_mut().append_pair("status", content);
        let request = url::Url::parse(&request.to_string().replace("+", "%20"))?;

        Ok(self.send_request(request, "POST")?)
    }

    /// Get tweet with id = `tweet_id`
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn get_tweet(&self, tweet_id: &str) -> Result<tweet_structure::Tweet, Box<Error>> {
        let mut request = url::Url::parse("https://api.twitter.com/1.1/statuses/show.json")?;
        request.query_pairs_mut().append_pair("id", tweet_id);

        Ok(self.send_request(request, "GET")?)
    }

    /// Get tweet that satisfy `query`
    /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
    pub fn get_tweets_query(
        &self,
        query: &str,
        options: tweet_structure::QueryOption,
    ) -> Result<Vec<tweet_structure::Tweet>, Box<Error>> {
        let mut request = url::Url::parse("https://api.twitter.com/1.1/search/tweets.json")?;
        request.query_pairs_mut().append_pair("q", query);
        request.query_pairs_mut().append_pair("count", "100");
        if let Some(result_type) = options.result_type {
            request
                .query_pairs_mut()
                .append_pair("result_type", &result_type);
        }
        if let Some(max_id) = options.max_id {
            request
                .query_pairs_mut()
                .append_pair("max_id", &max_id.to_string());
        }
        if let Some(since_id) = options.since_id {
            request
                .query_pairs_mut()
                .append_pair("since_id", &since_id.to_string());
        }
        let request = url::Url::parse(&request.to_string().replace("+", "%20"))?;

        let response: tweet_structure::SearchResponse = self.send_request(request, "GET")?;

        Ok(response.statuses)
    }
}

#[derive(Debug)]
pub struct TwitterBotError(String);

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
