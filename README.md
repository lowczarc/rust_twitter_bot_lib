[![](https://img.shields.io/crates/v/rust_twitter_bot_lib.svg)](https://crates.io/crates/rust_twitter_bot_lib)

# Rust Twitter Bot Lib
A Rust lib for creating Twitter Bot

## Usage
Add twitter_bot_rust in your project dependencies
```toml
[dependencies]
rust_twitter_bot_lib = "0.1.2"
```

And Tweet 😄
```rust
use rust_twitter_bot_lib::*;

fn main() {
  let example_bot = TwitterBot::new()
    .consumer_key(YOUR_CONSUMER_KEY)
    .consumer_secret_key(YOUR_CONSUMER_SECRET_KEY)
    .access_token(YOUR_ACCESS_TOKEN)
    .secret_access_token(YOUR_SECRET_ACCESS_TOKEN);

  let res = example_bot.tweet("🐦 + 🦀 = 💙 #myfirstTweet", None).unwrap();

  println!("{:?}", res);
}
```
<br/>

**Congratulations ! 🎉**  
![Twitter First Bot](https://image.noelshack.com/fichiers/2019/17/5/1556304403-screen-shot-2019-04-26-at-8-44-01-pm.png)
