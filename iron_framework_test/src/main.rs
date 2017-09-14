extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {

    fn my_endpoint(_: &mut Request) -> IronResult<Response> {
        println!("Request received!");
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    Iron::new(my_endpoint).http("localhost:3000").unwrap();
    println!("Listening on 3000");
}
