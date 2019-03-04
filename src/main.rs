#![feature(async_await, futures_api)]
use std::io::{stdin, Read};

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use futures::future::FutureObj;
use tide::{head::{Path, Named}, middleware::RequestContext, ExtractConfiguration, Response};

use self::models::*;
use self::diesel::prelude::*;

mod database;

#[derive(Clone, Debug, Default)]
struct uuid(String);

async fn get_talk() -> String {
    format!("Hello, world!")
}

fn debug_store(ctx: RequestContext<()>) -> FutureObj<Response> {
    println!("{:#?}", ctx.store());
    ctx.next()
}

use self::models::{Talk, NewTalk};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, description: &'a str) -> Talk {
    use schema::talks;

    let new_talk = NewTalk {
        title: title,
        description: description,
    };

    diesel::insert_into(talks::table)
        .values(&new_talk)
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    use schema::talks::dsl::*;

    let mut app = tide::App::new(());

    let connection = database::establish_connection();

    println!("What would you like your title to be?");
    let mut new_title = String::new();
    stdin().read_line(&mut new_title).unwrap();
    let new_title = &new_title[..(new_title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {}\n", new_title);
    let mut new_description = String::new();
    stdin().read_to_string(&mut new_description).unwrap();

    let post = create_post(&connection, new_title, &new_description);
    println!("\nSaved draft {} with id {}", new_title, post.id);


    let results = talks.filter(published.eq(true))
        .limit(5)
        .load::<Talk>(&connection)
        .expect("Error loading talks");

    println!("Displaying {} talks", results.len());
    for talk in results {
        println!("{}", talk.title);
        println!("----------\n");
        println!("{}", talk.description);
    }

    app.middleware(debug_store);
    app.at("/talks/{}").get(get_talk);

    app.serve();
}
