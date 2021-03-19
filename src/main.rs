#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rust API"
}
#[get("/greet/<name>")]
fn greet(name: String) -> String {
    format!("Hey {}, glad to have you here!", name)
}
fn main() {
    rocket::ignite().mount("/", routes![index,greet]).launch();
}
