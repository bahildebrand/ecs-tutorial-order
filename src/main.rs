#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> String {
    String::from("World")
}

fn main() {
    rocket::ignite().mount("/", routes![world]).launch();
}