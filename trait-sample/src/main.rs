use trait_sample::Tweet;
use trait_sample::Summary;

fn main() {
    let tweet = Tweet {
        username: String::from("tomoki"),
        content: String::from("今日はいい天気ですね。"),
        reply: false,
        retweet: true,
    };

    println!("1件の新しいツイート: {}", tweet.summarize());
}
