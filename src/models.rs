use super::schema::blog_posts;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::Queryable;
use diesel::Insertable;
use diesel::AsChangeset;


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "blog_posts"]
pub struct NewBlogPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "blog_posts"]
pub struct UpdatedBlogPost<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub published: Option<bool>,
}
