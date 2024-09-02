use ::entity::post;
use sea_orm::*;
use crate::post::repository::PostRepository;

pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<post::Model>, DbErr> {
        PostRepository::find_post_by_id(db, id).await
    }

    /// If ok, returns (post models, num pages).
    pub async fn find_posts_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64), DbErr> {
        PostRepository::find_posts_in_page(db, page, posts_per_page).await
    }
}
