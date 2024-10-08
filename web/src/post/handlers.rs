use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Html,
};
use domain::post::model::{Post, PostForm};
use domain::post::service::PostService;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{
    flash::{get_flash_cookie, post_response, PostResponse},
    AppState,
};

#[derive(Deserialize)]
pub struct Params {
    pub page: Option<u64>,
    pub posts_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlashData {
    pub kind: String,
    pub message: String,
}

pub async fn list_posts(
    state: State<AppState>,
    Query(params): Query<Params>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let page = params.page.unwrap_or(1);
    let posts_per_page = params.posts_per_page.unwrap_or(5);

    let post_service = PostService::new(&*state.repository.post);
    let (posts, num_pages) = post_service
        .find_posts_in_page(page, posts_per_page)
        .await
        .expect("Cannot find posts in page");

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &posts);
    ctx.insert("page", &page);
    ctx.insert("posts_per_page", &posts_per_page);
    ctx.insert("num_pages", &num_pages);

    if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
        ctx.insert("flash", &value);
    }

    let body = state
        .templates
        .render("post/index.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

pub async fn new_post(state: State<AppState>) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("post/new.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

pub async fn create_post(
    state: State<AppState>,
    mut cookies: Cookies,
    form: Form<PostForm>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    println!("In create post");
    let form = form.0;

    let post_service = PostService::new(&*state.repository.post);
    post_service
        .create_post(form)
        .await
        .expect("could not insert post");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Post successfully added".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}

pub async fn edit_post(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>, (StatusCode, String)> {
    let post_service = PostService::new(&*state.repository.post);
    let post: Post = post_service
        .find_post_by_id(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut ctx = tera::Context::new();
    ctx.insert("post", &post);

    let body = state
        .templates
        .render("post/edit.html.tera", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(body))
}

pub async fn update_post(
    state: State<AppState>,
    Path(id): Path<i32>,
    mut cookies: Cookies,
    form: Form<PostForm>,
) -> Result<PostResponse, (StatusCode, String)> {
    let form = form.0;

    let post_service = PostService::new(&*state.repository.post);
    post_service
        .update_post_by_id(id, form)
        .await
        .expect("could not edit post");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Post successfully updated".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}

pub async fn delete_post(
    state: State<AppState>,
    Path(id): Path<i32>,
    mut cookies: Cookies,
) -> Result<PostResponse, (StatusCode, String)> {
    let post_service = PostService::new(&*state.repository.post);
    match post_service.delete_post(id).await {
        Ok(_) => {
            let data = FlashData {
                kind: "success".to_owned(),
                message: "Post successfully deleted".to_owned(),
            };
            Ok(post_response(&mut cookies, data))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
