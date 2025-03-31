#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod routes;
mod models;
mod schema;
mod utils;

use rocket::Rocket;
use rocket::fairing::AdHoc;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

#[database("postgres_db")]
struct DbConn(PgConnection);

#[launch]
fn rocket() -> Rocket {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", run_migrations))
        .mount("/", routes![routes::index])
}

async fn run_migrations(rocket: Rocket) -> Rocket {
    let conn = DbConn::get_one(&rocket).await.expect("database connection");
    embedded_migrations::run(&conn).expect("database migrations");
    rocket
}
