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
    use crate::models::{PostInsert, Post};

    let id = 12;
    let post = diesel::update(posts::dsl::posts.find(id))
        .set(posts::published.eq(false))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("po updejcie {:?}", post)

    // let new_post = PostInsert {
    //     title: "dupa blada",
    //     body: "body body body",
    //     published: true,
    //     counter: 1,
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



    // let results = posts::dsl::posts
    //     //.filter(published.eq(true))
    //     .limit(20)
    //     .load::<Post>(&connection)
    //     .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("----------\n");
    //     println!("{}", post.title);
    //     println!("{}", post.body);
    //     println!("----------\n");
    // }
}
