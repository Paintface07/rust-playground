extern crate diesel_sample;
extern crate diesel;

use self::diesel_sample::*;
use self::diesel_sample::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_sample::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
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