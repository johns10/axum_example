use entity::post;
use sea_orm::ActiveValue;
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
            id: match active_model.id {
                ActiveValue::Set(id) => id,
                ActiveValue::Unchanged(id) => id,
                _ => Default::default(),
            },
            title: match active_model.title {
                ActiveValue::Set(title) => title,
                ActiveValue::Unchanged(title) => title,
                _ => Default::default(),
            },
            text: match active_model.text {
                ActiveValue::Set(text) => text,
                ActiveValue::Unchanged(text) => text,
                _ => Default::default(),
            },
        }
    }
}
