extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    let (value1, value2) = ("Hello", "world");
    let value3 : i32 = 42;
    let value4 : i32 = 22;
    let value5 : i32 = add_numbers(value3, value4);
//    let value6 : [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//    let my_tuple : (i32, i32, i32) = (1, 2, 3);

    println!("{} {}!", value1, value2);
    println!("My favorite number is {}.", value5);
    // println!("My Array: {}" + &value6[1..1]);

    fn my_endpoint(_: &mut Request) -> IronResult<Response> {
        println!("Request received!");
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    Iron::new(my_endpoint).http("localhost:3000").unwrap();
    println!("Listening on 3000");
}

fn add_numbers(x : i32, y : i32) -> i32 {
    x + y
}