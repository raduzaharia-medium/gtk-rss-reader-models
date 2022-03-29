use glib::{
    object_subclass,
    once_cell::sync::Lazy,
    subclass::{object::ObjectImpl, types::ObjectSubclass},
    ParamFlags, ParamSpec, ParamSpecString, ToValue, Value,
};
use std::cell::Cell;

// Object holding the state
#[derive(Default)]
pub struct FeedItemTemplate {
    name: Cell<String>,
    url: Cell<String>,
}

#[object_subclass]
impl ObjectSubclass for FeedItemTemplate {
    const NAME: &'static str = "FeedItem";
    type Type = super::FeedItem;
}

impl ObjectImpl for FeedItemTemplate {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::new("name", "name", "The name of the RSS feed", Some(""), ParamFlags::READWRITE),
                ParamSpecString::new("url", "url", "The url of the RSS feed", Some(""), ParamFlags::READWRITE),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let name_string = value.get().expect("The value needs to be of type `String`.");
                self.name.replace(name_string);
            }
            "url" => {
                let url_string = value.get().expect("The value needs to be of type `String`.");
                self.url.replace(url_string);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.name.take().to_value(),
            "url" => self.url.take().to_value(),
            _ => unimplemented!(),
        }
    }
}
