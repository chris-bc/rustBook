use blog::Post;

fn main() {
    let mut post = Post::new();

    assert!(post.add_text("I ate a salad for lunch today").is_ok());
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // Should not be able to add text in review
    assert!(post.add_text("Some text").is_err());

    post.reject();
    assert_eq!("", post.content());

    // Back in draft and should be able to edit again
    assert!(post.add_text("More text").is_ok());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch todayMore text", post.content());
}