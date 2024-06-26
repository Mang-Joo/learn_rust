use learn_rust::{NewsArticle, notify, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize_author());

    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", news.summarize());
    println!("New article available! {}", news.summarize_author());

    notify(&tweet);
    notify(&news);


}
