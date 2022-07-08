#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::PgConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use std::env;

mod constants;
mod models;
mod repositories;
mod schema;
mod services;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // .configure(services::init)
            .service(web::scope("/api").configure(services::init))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
