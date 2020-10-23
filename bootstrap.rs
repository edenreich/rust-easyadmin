#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate easyadmin;
extern crate r2d2;
extern crate r2d2_diesel;

mod controllers;
mod database;
mod models;

use controllers::{admin, auth, frontend};
use database::connection;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;

fn main() {
    Rocket::ignite()
        .manage(connection::init_pool())
        .mount("/admin", routes![admin::home_controller::index])
        .mount("/dashboard", routes![admin::home_controller::dashboard])
        .mount(
            "/auth",
            routes![
                auth::login_controller::form,
                auth::login_controller::submit,
                auth::register_controller::form,
                auth::register_controller::submit
            ],
        )
        .mount("/", routes![frontend::home_controller::index])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
        .launch();
}
