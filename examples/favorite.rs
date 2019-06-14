use rust_twitter_bot_lib::*;

fn main() {
    let example_bot = TwitterBot::new()
        .consumer_key(YOUR_CONSUMER_KEY)
        .consumer_secret_key(YOUR_CONSUMER_SECRET_KEY)
        .access_token(YOUR_ACCESS_TOKEN)
        .secret_access_token(YOUR_SECRET_ACCESS_TOKEN);

    let res_retweet = example_bot
        .retweet(1122130188203171840);

    let res_favorite = example_bot
        .favorite(1122130188203171840);

    println!("{:?}\n{:?}\n", res_retweet, res_favorite)
}
