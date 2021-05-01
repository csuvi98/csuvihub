#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


mod hub;
mod panel;
mod tutorial;
mod page;

use crate::hub::hub::Hub;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    let hub : Hub;
    //rocket::ignite().mount("/", routes![hello]).launch();
}
