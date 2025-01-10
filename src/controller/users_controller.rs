use actix_web::{get, post, web, HttpResponse, Responder};

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use crate::entity::user_entity::NewUser;
use crate::service::user_service::UserService;


// Tipo del pool de conexión a la base de datos
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn list_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Llamar al servicio para obtener los usuarios
    match UserService::get_all_users(&mut *conn) {
        Ok(users) => HttpResponse::Ok().json(users), // Devolver la lista de usuarios en formato JSON
        Err(e) => {
            eprintln!("Error creating user: {:?}", e); // Imprimir el error en consola
            HttpResponse::InternalServerError().body(format!("Error creating user: {}", e))

        }

    }
}

#[post("/user")]
async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match UserService::create_user(&mut conn, &user.into_inner()) {
        Ok(created_user) => HttpResponse::Created().json(created_user),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e); // Imprimir el error en consola
            HttpResponse::InternalServerError().body(format!("Error creating user: {}", e))
        }
    }
}

// Función para inicializar las rutas
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
        .service(create_user);
}

