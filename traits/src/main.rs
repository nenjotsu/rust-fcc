mod where_generics;
mod condiionally;
mod blanket_impl;

pub struct NewsArticle {
  pub author: String,
  pub headline: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {}", self.headline, self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
 
}

// fn summarize(&self) -> String {
//   format!("{}, by {}", self.username, self.content)
// }

pub trait Summary {
  // without default implementation
  // fn summarize(&self) -> String;
  // default implementation
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

fn main() {
  let tweet = Tweet {
    username: String::from("@johndoe"),
    content: String::from("hellow world"),
    reply: false,
    retweet: false,
  };

  let article = NewsArticle {
    author: String::from("John Doe"),
    headline: String::from("The sky is falling"),
    content: String::from("the sky is not a ctually falling")
  };

  println!("tweet summary: {}", tweet.summarize());
  println!("article summary: {}", article.summarize());
}
