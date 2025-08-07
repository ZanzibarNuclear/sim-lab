pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(
            "Open this beauty by {}. Do it now!!!",
            self.summarize_author()
        )
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("<{}>", self.author)
    }
    fn summarize(&self) -> String {
        format!(
            "{}, by {} (in {})",
            self.headline,
            self.summarize_author(),
            self.location
        )
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Same as it ever was: {}", item.summarize());
}
