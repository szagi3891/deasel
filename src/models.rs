use diesel::{Queryable, Insertable};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


// #[derive(Insertable)]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }

use super::schema::posts;

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost2<'a> {
    pub title: &'a str,
}