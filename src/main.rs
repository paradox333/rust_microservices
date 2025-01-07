use actix_web::{App, HttpServer};
mod controller;

use controller::users_controller::init_routes; // Import init_routes

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init_routes) // Use init_routes to configure the routes
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

