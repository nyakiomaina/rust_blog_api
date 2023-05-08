use actix_web::{App, HttpServer, web, HttpResponse};
use dotenv::dotenv;
use std::io;
use diesel::result::Error as DieselError;
extern crate diesel;

mod models;
mod schema;
mod handlers;
mod db;
pub mod actions;

use db::DbPool;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let pool = db::init_pool();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/blog_posts", web::get().to(handlers::get_blog_posts))
            .route("/blog_posts", web::post().to(handlers::create_blog_post))
            .route("/blog_posts/{id}", web::get().to(handlers::get_blog_post))
            .route("/blog_posts/{id}", web::put().to(handlers::update_blog_post))
            .route("/blog_posts/{id}", web::delete().to(handlers::delete_blog_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Convert Diesel error to actix_web::Error
impl From<DieselError> for actix_web::Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => {
                actix_web::error::ErrorInternalServerError(err)
            }
            DieselError::NotFound => actix_web::error::ErrorNotFound("Record not found"),
            _ => actix_web::error::ErrorInternalServerError(error),
        }
    }
}
