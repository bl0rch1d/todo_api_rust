#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod schema;
mod models;
mod db_connection;
mod routes;
mod operations;

use crate::db_connection::DbConnection;
use routes::*;

fn main() {
    rocket::ignite()
        .attach(DbConnection::fairing())
        .mount("/", routes![list_projects, create_project, update_project, delete_project, show_project])
        .launch();
}
