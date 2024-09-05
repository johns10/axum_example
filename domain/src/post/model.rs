use entity::post;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostForm {
    pub title: String,
    pub text: String,
}

impl From<post::Model> for Post {
    fn from(model: post::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            text: model.text,
        }
    }
}

impl From<post::ActiveModel> for Post {
    fn from(active_model: post::ActiveModel) -> Self {
        Self {
            id: active_model.id.unwrap_or_default(),
            title: active_model.title.unwrap_or_default(),
            text: active_model.text.unwrap_or_default(),
        }
    }
}
