use crate::actions;
use crate::db::DbPool;
use crate::models::{BlogPost, NewBlogPost, UpdatedBlogPost};
use actix_web::error::ErrorInternalServerError;
use diesel::prelude::*;
use std::convert::TryInto;
use actix_web::{web};
use actix_web::web::Path;
use actix_web::web::Json;


pub async fn get_all_blog_posts(pool: &DbPool) -> Result<Vec<BlogPost>, actix_web::Error> {
    let conn = pool.get().map_err(ErrorInternalServerError)?;
    let blog_posts = web::block(move || actions::get_all_blog_posts(&conn))
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(blog_posts)
}

pub async fn get_blog_post(id: &Path<i32>, pool: &DbPool) -> Result<BlogPost, actix_web::Error> {
    let conn = pool.get().map_err(ErrorInternalServerError)?;
    let post_id = id.into_inner();
    let blog_post = web::block(move || actions::get_blog_post(&conn, post_id))
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| actix_web::error::ErrorNotFound(format!("Blog post {} not found", post_id)))?;
    Ok(blog_post)
}

pub async fn create_blog_post(new_blog_post: &Json<NewBlogPost<'_>>, pool: &DbPool) -> Result<BlogPost, actix_web::Error> {
    let conn = pool.get().map_err(ErrorInternalServerError)?;
    let new_post: NewBlogPost = new_blog_post.into_inner().try_into().map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    let blog_post = web::block(move || actions::create_blog_post(&conn, &new_post))
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(blog_post)
}

pub async fn update_blog_post(id: &Path<i32>, updated_blog_post: &Json<UpdatedBlogPost<'_>>, pool: &DbPool) -> Result<BlogPost, actix_web::Error> {
    let conn = pool.get().map_err(ErrorInternalServerError)?;
    let post_id = id.into_inner();
    let updated_post: UpdatedBlogPost = updated_blog_post.into_inner().try_into().map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    let blog_post = web::block(move || actions::update_blog_post(&conn, post_id, &updated_post))
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| actix_web::error::ErrorNotFound(format!("Blog post {} not found", post_id)))?;
Ok(blog_post)
}
pub async fn delete_blog_post(id: &Path<i32>, pool: &DbPool) -> Result<(), actix_web::Error> {
    let conn = pool.get().map_err(ErrorInternalServerError)?;
    let post_id = id.into_inner();
    let deleted = web::block(move || actions::delete_blog_post(&conn, post_id))
    .await
    .map_err(ErrorInternalServerError)?;
    
    if deleted {
        Ok(())
    } else {
        Err(actix_web::error::ErrorNotFound(format!("Blog post {} not found", post_id)))
    }
}  