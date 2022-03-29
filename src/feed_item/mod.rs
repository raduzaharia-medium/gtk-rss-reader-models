mod template;

use glib::Object;
use template::FeedItemTemplate;

glib::wrapper! {
    pub struct FeedItem(ObjectSubclass<FeedItemTemplate>);
}

impl FeedItem {
    pub fn new(name: &str, url: &str) -> Self {
        Object::new(&[("name", &name), ("url", &url)]).expect("Failed to create `FeedItem`.")
    }
}
