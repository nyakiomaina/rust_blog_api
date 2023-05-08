use actix_web::{HttpResponse, Responder, web, Error};
use serde_json::json;
use crate::db::DbPool;
use crate::actions;
use crate::models::{BlogPost, NewBlogPost};
use actix_web::web::{Data, Json, Path};

pub async fn get_blog_posts(pool: Data<DbPool>) -> Result<impl Responder, Error> {
    let conn = pool.get().unwrap();
    let posts = web::block(move || actions::get_all_blog_posts(&conn))
        .await
        .map_err(|_| HttpResponse::InternalServerError().into())?;

    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_blog_post(id: Path<i32>, pool: Data<DbPool>) -> Result<impl Responder, Error> {
    let conn = pool.get().unwrap();
    let post = web::block(move || actions::get_blog_post(&conn, id.into_inner()))
        .await
        .map_err(|_| HttpResponse::InternalServerError().into())?;

    if let Some(post) = post {
        Ok(HttpResponse::Ok().json(post))
    } else {
        Err(HttpResponse::NotFound().into())
    }
}

pub async fn create_blog_post<'a>(new_blog_post: Json<NewBlogPost<'a>>, pool: Data<DbPool>) -> Result<impl Responder, Error> {
    let conn = pool.get().unwrap();
    let post = web::block(move || actions::create_blog_post(&conn, &new_blog_post))
        .await
        .map_err(|_| HttpResponse::InternalServerError().into())?;

    Ok(HttpResponse::Created().json(post))
}

pub async fn update_blog_post(id: Path<i32>, updated_blog_post: Json<BlogPost>, pool: Data<DbPool>) -> Result<impl Responder, Error> {
    let conn = pool.get().unwrap();
    let post = web::block(move || actions::update_blog_post(&conn, id.into_inner(), &updated_blog_post))
        .await
        .map_err(|_| HttpResponse::InternalServerError().into())?;

    if let Some(post) = post {
        Ok(HttpResponse::Ok().json(post))
    } else {
        Err(HttpResponse::NotFound().into())
    }
}

pub async fn delete_blog_post(id: Path<i32>, pool: Data<DbPool>) -> Result<impl Responder, Error> {
    let conn = pool.get().unwrap();
    let deleted = web::block(move || actions::delete_blog_post(&conn, id.into_inner()))
        .await
        .map_err(|_| HttpResponse::InternalServerError().into())?;

    if deleted {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(HttpResponse::NotFound().into())
    }
}
