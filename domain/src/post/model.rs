use entity::post;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub text: String,
}

impl From<post::Model> for Post {
    fn from(model: post::Model) -> Self {
        Self {
            id: Some(model.id),
            title: model.title,
            text: model.text,
        }
    }
}
