mod lib;
// Traits work when they are in scope
use lib::{notify, Summary, Summary3};

fn main() {
    let tweet = lib::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = lib::NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!(
        "{},{},{}",
        tweet.summarize(),
        lib::Summary2::summarize(&article),
        lib::Summary::summarize(&article)
    );

    println!("----------------------------------------------------------");

    println!("1 new tweet: {}", tweet.summarizer());

    println!("----------------------------------------------------------");

    notify(&tweet);
}
