use super::schema::blog_posts;
use chrono::NaiveDateTime;
use serde::Serialize;
use diesel::Queryable;
use diesel::Insertable;
use diesel::AsChangeset;


#[derive(Queryable, Serialize)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "blog_posts"]
pub struct NewBlogPost<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "blog_posts"]
pub struct UpdatedBlogPost<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub updated_at: NaiveDateTime,
}
