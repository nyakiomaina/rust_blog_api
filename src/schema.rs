use diesel::table;
// use diesel::types::{Int4, Text, Timestamp, Varchar};

table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
