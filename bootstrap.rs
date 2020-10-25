#![feature(proc_macro_hygiene, decl_macro)]

extern crate easyadmin;

mod models;
mod controllers;
mod database;

use easyadmin::thirdparty::{rocket, rocket::routes, rocket::Rocket, rocket_contrib::serve::StaticFiles};

use controllers::{admin, auth, frontend};
use database::connection;

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
