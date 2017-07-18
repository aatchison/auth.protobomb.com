extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request)  -> IronResult<Response> {
        Ok(Response::with((status::Ok, "protobomb\nprimordial soups\nrecipe futures\n\nwritten_in_rust")))
    }

    let _server = Iron::new(hello_world).http("0.0.0.0:5000").unwrap();
    println!("Serving on 5000");
}
