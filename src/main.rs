#![feature(async_await, futures_api)]
#[macro_use]

use diesel;
use dotenv;

use futures::future::FutureObj;
use tide::{head::{Path, Named}, middleware::RequestContext, ExtractConfiguration, Response};

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

fn main() {
    let mut app = tide::App::new(());

    database::establish_connection();

    app.middleware(debug_store);
    app.at("/talks/{}").get(get_talk);

    app.serve();
}
