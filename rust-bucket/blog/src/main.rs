use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("**in draft**", post.content());

    post.add_text(" Wow!");
    assert_eq!("**in draft**", post.content());

    post.request_review();
    assert_eq!("**pending review**", post.content());

    post.add_text(" Wow!");
    post.reject();
    assert_eq!("**in draft**", post.content());

    post.request_review();
    assert_eq!("**pending review**", post.content());

    post.approve();
    assert_eq!("**pending review**", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today Wow!", post.content());

    post.add_text("Blargy pants");
    assert_eq!("I ate a salad for lunch today Wow!", post.content());
}
