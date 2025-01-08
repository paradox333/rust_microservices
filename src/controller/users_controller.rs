use actix_web::{get, post, web, HttpResponse, Responder};

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use crate::service::user_service::UserService;


// Tipo del pool de conexión a la base de datos
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn list_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Llamar al servicio para obtener los usuarios
    match UserService::get_all_users(&mut *conn) {
        Ok(users) => HttpResponse::Ok().json(users), // Devolver la lista de usuarios en formato JSON
        Err(_) => HttpResponse::InternalServerError().body("Error fetching users"),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Función para inicializar las rutas
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
        .service(echo);
}

