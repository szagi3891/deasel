#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

/*
docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d -p 5432:5432 postgres
*/

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {

    println!("Hello, world!");

    let connection = establish_connection();

    use crate::schema::posts;   //::dsl::*;
    use crate::models::{NewPost, NewPost2, Post};

    // let new_post = NewPost2 {
    //     title: "dupa blada",
    //     //body: "body body body",
    // };

    // // let new_post = NewPost {
    // //     title: "dupa blada",
    // //     body: "body body body",
    // // };

    // let aa: Post = diesel::insert_into(posts::table)
    //     .values(&new_post)
    //     .get_result(&connection)
    //     .expect("Error saving new post");

    // println!("Wstawiono coś do bazy danych ... {:?}", aa.id);


    let results = posts::dsl::posts
        //.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
