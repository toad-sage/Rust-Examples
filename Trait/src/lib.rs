pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Summary2 {
    // default implementation
    fn summarize(&self) -> String {
        String::from("(Read More...)")
    }
}

pub trait Summary3 {
    fn summarize_author(&self) -> String;
    fn summarizer(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary2 for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> std::string::String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary3 for Tweet {
    fn summarize_author(&self) -> std::string::String {
        format!("@{}", self.username)
    }
}

// items are those which implement trait Summary3

pub fn notify(item: &impl Summary3) {
    println!("Breaking news! {}", item.summarizer());
}

// or

pub fn notify2<T: Summary3>(item: &T) {
    println!("Breaking news! {}", item.summarizer());
}

// or

pub fn notify3<T>(item: &T)
where
    T: Summary3,
{
    println!("Breaking news! {}", item.summarizer());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

//you can only use impl Trait if you’re returning a single type
// fn returns_summarizable1(switch: bool) -> impl Summary {
//     if switch {
//         return NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
// hockey team in the NHL.",
//             ),
//         }
//     } else {
//         return Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
