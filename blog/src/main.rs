use blog::Post;

fn main() {
    let mut post = Post::new();

    let res = post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    if let Ok(_) = res {
        assert!(true);
    } else {
        assert!(false);
    }

    post.request_review();
    assert_eq!("", post.content());

    // Should not be able to add text in review
    if let Err(_) = post.add_text("Some text") {
        assert!(true);
    } else {
        assert!(false);
    }

    post.reject();
    assert_eq!("", post.content());

    // Back in draft and should be able to edit again
    if let Ok(_) = post.add_text("More text") {
        assert!(true);
    } else {
        assert!(false);
    }

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch todayMore text", post.content());
}