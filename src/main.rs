use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod models;
mod schema;
mod handlers;
mod db;
mod actions;

use db::DbPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
