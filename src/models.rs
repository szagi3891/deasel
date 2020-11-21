use diesel::{Queryable, Insertable};

use super::schema::posts;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub counter: i32,
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="posts"]
pub struct PostInsert<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
    pub counter: i32,
}
