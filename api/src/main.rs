#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

mod config;
mod controllers;
mod models;
mod twitter_api;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(controllers::artist_controller::stage())
}