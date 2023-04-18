#[cfg(test)]
#[test]
fn test_post_fields() {
    use crate::models::post::Post;
    use std::mem;

    let post = Post {
        id: 1,
        title: String::from("Test Post"),
        body: String::from("This is a test post."),
        published: false,
    };

    assert_eq!(post.id, 1);
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.body, "This is a test post.");
    assert_eq!(post.published, false);
    let num_fields = mem::size_of::<Post>() / mem::size_of::<i32>();
    assert_eq!(num_fields, 14);
}
