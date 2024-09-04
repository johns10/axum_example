use crate::post::model::Post;
use ::entity::post;
use sea_orm::*;

pub struct PostService;

impl PostService {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<Post>, DbErr> {
        post::Entity::find_by_id(id)
            .one(db)
            .await
            .map(|opt_model| opt_model.map(Post::from))
    }

    pub async fn find_posts_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<Post>, u64), DbErr> {
        let paginator = post::Entity::find()
            .order_by_asc(post::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| {
            (
                p.into_iter()
                    .map(|model| Post {
                        id: Some(model.id),
                        title: model.title,
                        text: model.text,
                    })
                    .collect(),
                num_pages,
            )
        })
    }

    pub async fn create_post(db: &DbConn, form_data: Post) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            id: ActiveValue::NotSet,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        form_data: Post,
    ) -> Result<post::Model, DbErr> {
        let post: post::ActiveModel = post::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post::ActiveModel {
            id: post.id,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = post::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        post::Entity::delete_many().exec(db).await
    }
}
