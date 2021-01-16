#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/order/<item>")]
fn order(item: String) -> String {
    format!("Item: {}", item)
}

#[get("/order/<item>/status")]
fn status(item: String) -> String {
    format!("{} status", item)
}

fn main() {
    rocket::ignite().mount("/", routes![order, status]).launch();
}