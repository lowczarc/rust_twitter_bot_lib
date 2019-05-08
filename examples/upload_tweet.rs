use rust_twitter_bot_lib::*;
use std::path::Path;

fn main() {
    let example_bot = TwitterBot::new()
        .consumer_key(YOUR_CONSUMER_KEY)
        .consumer_secret_key(YOUR_CONSUMER_SECRET_KEY)
        .access_token(YOUR_ACCESS_TOKEN)
        .secret_access_token(YOUR_SECRET_ACCESS_TOKEN);

    let media_id = example_bot
        .upload_file(Path::new("examples/chess.png"))
        .unwrap();
    let res = example_bot
        .tweet("Je teste des trucs", Some(media_id))
        .unwrap();

    println!("{:?}", res);
}
