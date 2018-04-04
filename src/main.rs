#![feature(decl_macro, plugin)]
// #![feature(alloc_system)]
#![plugin(rocket_codegen)]

// extern crate alloc_system;
extern crate csv;
extern crate rocket;
// extern crate serde;

// use std::time::Duration;
// use std::thread;

// use serde::Serialize;

fn get_big_string(x: usize) -> String {
    // let updates = vec![("test", 1, 2.3); x];
    // let output: Vec<u8> = Vec::new();

    // let mut wtr = csv::Writer::from_writer(output);
    // wtr.write_record(&["a", "b", "c"]).unwrap();
    // for val in updates {
    //     wtr.serialize(val).unwrap();
    // }

    // String::from_utf8(wtr.into_inner().unwrap()).unwrap()

    String::from_utf8(vec!['a' as u8; x]).unwrap()
}

#[get("/<x>")]
fn index(x: usize) -> Result<String, String> {
    Ok(get_big_string(x))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();

    // let it = 1;
    // loop {
    //     let x = get_big_string(it + 100000);
    //     thread::sleep(Duration::from_secs(2));
    //     println!("{}", x.len());
    // }
}
