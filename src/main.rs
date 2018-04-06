#![feature(decl_macro, plugin)]
#![feature(alloc_system, global_allocator, allocator_api)]
#![plugin(rocket_codegen)]

use std::thread;
use std::time::Duration;

extern crate alloc_system;
extern crate rocket;

use rocket::http::Status;
use rocket::local::Client;

use std::heap::System;

#[global_allocator]
static ALLOCATOR: System = System;

fn get_big_string(x: usize) -> String {
    String::from_utf8(vec!['a' as u8; x]).unwrap()
}

#[get("/<x>")]
fn index(x: usize) -> Result<String, String> {
    Ok(get_big_string(x))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();

    // let rocket = rocket::ignite().mount("/", routes![index]);
    // let client = Client::new(rocket).expect("valid rocket");

    // loop {
    //     println!("Making 25MB request...");
    //     let res = client.get("/25000000").dispatch();
    //     assert_eq!(res.status(), Status::Ok);

    //     thread::sleep(Duration::from_secs(1));
    // }
}
