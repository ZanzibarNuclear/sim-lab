pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            review_cnt: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    review_cnt: u32,
}

const REQUIRED_APPROVALS: u32 = 5;

pub enum ApprovalResult {
    Pending(PendingReviewPost),
    Approved(Post),
}

impl PendingReviewPost {
    pub fn approve(mut self) -> ApprovalResult {
        self.review_cnt += 1;
        println!("Has {} approvals", self.review_cnt);
        if self.review_cnt >= REQUIRED_APPROVALS {
            ApprovalResult::Approved(Post {
                content: self.content,
            })
        } else {
            ApprovalResult::Pending(self)
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
