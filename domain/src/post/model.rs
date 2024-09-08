use chrono::NaiveDateTime;
use entity::posts;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostForm {
    pub title: String,
    pub text: String,
}

impl From<posts::Model> for Post {
    fn from(model: posts::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            text: model.text,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<posts::ActiveModel> for Post {
    fn from(active_model: posts::ActiveModel) -> Self {
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
            created_at: match active_model.created_at {
                ActiveValue::Set(created_at) => created_at,
                ActiveValue::Unchanged(created_at) => created_at,
                _ => Default::default(),
            },
            updated_at: match active_model.updated_at {
                ActiveValue::Set(updated_at) => updated_at,
                ActiveValue::Unchanged(updated_at) => updated_at,
                _ => Default::default(),
            },
        }
    }
}
