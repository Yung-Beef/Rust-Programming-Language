pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//impl Summary for NewsArticle {} // implements the trait for the NewsArticle type, but will use the default implementation

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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author, self.content)
    }
}

// same as pub fn notify<T: Summary>(item: &T) {
pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // lets item1 and item2 have different types (both with Summary)
//pub fn notify<T: Summary>(item1: &T, item2: &T) {} // forces  both parameters to have the same type

//pub fn notify(item: &(impl Summary + Display)) {} // only accepts types that have both traits implemented
//pub fn notify<T: Summary + Display>(item: &T) {} // trait bounds on generic types

// hard to read with multiple generic types with multiple trait bounds
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// easier to read syntax using 'where'
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// returns a value of some type that has the Summary trait implemented
// in this case, it returns a Tweet, but the calling code doesn't need to know that
// can only return a single type, if the function could return either 1 type or 2nd type, it won't work like this
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    }
}


use std::fmt::Display;

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
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// standard libary that implements the ToString trait on any type that implements the Display trait
impl<T: Display> ToString or T {}