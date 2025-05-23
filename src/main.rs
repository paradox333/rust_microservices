use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;

mod controller;
use controller::users_controller::init_routes;

mod service;
mod entity;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("HOST not set in .env");
    let port = env::var("PORT").expect("PORT not set in .env");
    let address = format!("{}:{}", host, port);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Pasar el pool de conexión
            .configure(init_routes) // Configurar las rutas
    })
    .bind(address)?
    .run()
    .await
}

