#![recursion_limit="128"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


#[derive(Queryable)]
pub struct Message {
    pub message_id: String,
    pub channel_id: String,
    pub author: String,
    pub content: String,
    pub everyone: bool,
    pub ispinned: bool
}

// Probably want to look at http://diesel.rs/guides/getting-started/
fn main() {
//    use self::postgres_sample::messages::dsl::*;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set!");

//    infer_schema!("dotenv:DATABASE_URL");
//    infer_table_from_schema!("dotenv:DATABASE_URL", "public.message");
    table! {
        message (message_id) {
            message_id -> Varchar,
            channel_id -> Varchar,
            author -> Varchar,
            content -> Nullable<Varchar>,
            created -> Timestamp,
            edited -> Nullable<Timestamp>,
            everyone -> Bool,
            ispinned -> Bool,
        }
    }

    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let results = message.limit(10)
        .load::<Message>(&connection)
        .expect("Error loading messages.");
}
