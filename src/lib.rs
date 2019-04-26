extern crate oauthcli;
extern crate url;
extern crate reqwest;

use std::{fmt::{self, Display}, error::Error};

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
  
  /// Tweet `content`<br/>  
  /// Will fail if `consumer_key`, `consumer_key`, `access_token` and `secret_access_token` are not set
  pub fn tweet(&self, content: &str) -> Result<reqwest::Response, Box<Error>> {
    if self.consumer_key.is_none() {
      return Err(TwitterBotError("consumer_key missing").into());
    } else if self.consumer_secret_key.is_none() {
      return Err(TwitterBotError("consumer_secret_key missing").into());
    } else if self.access_token.is_none() {
      return Err(TwitterBotError("access_token missing").into());
    } else if self.secret_access_token.is_none() {
      return Err(TwitterBotError("secret_access_token missing").into());
    } 

    let mut request = url::Url::parse("https://api.twitter.com/1.1/statuses/update.json")?;
    request.query_pairs_mut().append_pair("status", content);
    let request = url::Url::parse(&request.to_string().replace("+", "%20"))?;

    let header = oauthcli::OAuthAuthorizationHeaderBuilder::new(
      "POST",
      &request,
      self.consumer_key.unwrap(),
      self.consumer_secret_key.unwrap(),
      oauthcli::SignatureMethod::HmacSha1
    )
      .token(self.access_token.unwrap(), self.secret_access_token.unwrap())
      .finish_for_twitter();

    let client = reqwest::Client::new();
    let res = client.post(&request.to_string())
      .header("Authorization", header.to_string())
      .send()?;
    Ok(res)
  }
}

#[derive(Debug)]
pub struct TwitterBotError(&'static str);

impl Display for TwitterBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TwitterBotError {
    fn description(&self) -> &str {
        self.0
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
