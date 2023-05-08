use diesel::prelude::*;
use crate::models::{BlogPost, NewBlogPost, UpdatedBlogPost};
use crate::schema::blog_posts::dsl::*;

pub fn get_all_blog_posts(conn: &SqliteConnection) -> QueryResult<Vec<BlogPost>> {
    blog_posts.load::<BlogPost>(conn)
}

pub fn get_blog_post(conn: &SqliteConnection, post_id: i32) -> QueryResult<Option<BlogPost>> {
    blog_posts.find(post_id).first::<BlogPost>(conn).optional()
}

pub fn create_blog_post(conn: &SqliteConnection, new_post: &NewBlogPost) -> QueryResult<BlogPost> {
    diesel::insert_into(blog_posts)
        .values(new_post)
        .execute(conn)?;

    blog_posts.order(created_at.desc()).first(conn)
}

pub fn update_blog_post(conn: &SqliteConnection, post_id: i32, updated_post: &UpdatedBlogPost) -> QueryResult<Option<BlogPost>> {
    diesel::update(blog_posts.find(post_id))
        .set((title.eq(&updated_post.title), content.eq(&updated_post.content), updated_at.eq(chrono::Local::now().naive_local())))
        .execute(conn)?;

    get_blog_post(conn, post_id)
}

pub fn delete_blog_post(conn: &SqliteConnection, post_id: i32) -> QueryResult<bool> {
    let rows_deleted = diesel::delete(blog_posts.find(post_id)).execute(conn)?;
    Ok(rows_deleted > 0)
}
