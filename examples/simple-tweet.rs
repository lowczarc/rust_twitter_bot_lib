use rust_twitter_bot_lib::*;

fn main() {
  let example_bot = TwitterBot::new()
    .consumer_key(YOUR_CONSUMER_KEY)
    .consumer_secret_key(YOUR_CONSUMER_SECRET_KEY)
    .access_token(YOUR_ACCESS_TOKEN)
    .secret_access_token(YOUR_SECRET_ACCESS_TOKEN);

  let mut res = example_bot.tweet("🐦 + 🦀 = 💙 #myfirstTweet").unwrap();

  println!("{}", res.text().unwrap());
}
