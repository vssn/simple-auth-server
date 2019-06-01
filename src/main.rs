#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate r2d2;
extern crate serde;
extern crate uuid;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate bcrypt;
extern crate env_logger;
extern crate jsonwebtoken as jwt;

mod app;
mod errors;
mod models;
mod handler;
mod routes;
mod schema;
mod utils;

use actix::prelude::*;
use actix_web::server;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use models::DbExecutor;
use std::env;

fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "simple-auth-server=debug,actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sys = actix::System::new("Actix_Tutorial");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    server::new(move || app::create_app(address.clone()))
        .bind("127.0.0.1:9080")
        .expect("Can not bind to 127.0.0.1:9080")
        .start();

    sys.run();
}
