mod lifetimes;

use std::fmt::Display;

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer.x = {}", integer.x());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let test = DefaultSummary {};
    println!("{}", test.summarize());

    notify(&tweet);
    notify2(&tweet);

    let test = returns_summarizable();

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // if lifetimes not equal, 'a indicates smaller lifetime
    // borrow checker
    // just need to pass to caller scope - 1 step up stack
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = lifetimes::longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // struct with lifetime and reference
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = lifetimes::ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime
    // string literals all have static lifetime regardless
    // using is a code smell
    let s: &'static str = "I have a static lifetime.";
}

// used in struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// only for f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub trait Summary {
    // body is optional, default implementation
    fn summarize(&self) -> String {
        String::from("(Read more...)")
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
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct DefaultSummary {}

impl Summary for DefaultSummary {}

// annotate type implementing a trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound, above is syntactic sugar for same thing
// only this method could ensure that two parameters were the exact same type
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// multiple trait bounds for same parameter can be done as follows:
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}

// where clause, alternative to
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
/*fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}*/

// return type implementing trait
// can only be done if returning a single type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// also can do blanket implementation to define trait for another trait