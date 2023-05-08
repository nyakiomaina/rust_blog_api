#[macro_use]
extern crate diesel;

table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(blog_posts,);
