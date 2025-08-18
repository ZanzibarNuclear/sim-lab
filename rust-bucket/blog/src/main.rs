use blog::{ApprovalResult, Post};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let mut pending = post.request_review();

    // Loop until the post is approved
    let post = loop {
        match pending.approve() {
            ApprovalResult::Approved(post) => break post,
            ApprovalResult::Pending(next_pending) => {
                println!("Approval needed...");
                pending = next_pending;
            }
        }
    };

    // post should be a Post, able to see content at this point
    assert_eq!("I ate a salad for lunch today", post.content());
}
