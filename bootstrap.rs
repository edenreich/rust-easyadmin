#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate easyadmin;

mod controllers;

use controllers::{admin, frontend, auth};
use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
        .mount("/admin", routes![admin::home_controller::index])
        .mount("/dashboard", routes![admin::home_controller::dashboard])
        .mount("/auth", routes![auth::login_controller::form, auth::login_controller::submit, auth::register_controller::form, auth::register_controller::submit])
        .mount("/", routes![frontend::home_controller::index])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
        .launch();
}
