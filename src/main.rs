#![feature(proc_macro_hygiene, decl_macro)]
mod routes;

#[macro_use]
extern crate rocket;

fn main() {
    rocket::ignite()
        .mount("/user", routes![routes::user::get_user])
        .launch();
}
