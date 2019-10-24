#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;
#[macro_use] extern crate failure;
extern crate chrono;

#[database("postgresql_logs")]
pub struct Db(diesel::PgConnection);

mod schema;
mod models;
mod controllers;
mod sw;

fn main() {
    let routes = routes![
        controllers::domains::index,
        controllers::machines::index,
        controllers::machines_program::index,
        controllers::programs::index,
        controllers::publishers::index,
        controllers::solar_winds::upload,
    ];

    rocket::ignite()
        .attach(Db::fairing())
        .mount("/api", routes)
        .launch();
}
