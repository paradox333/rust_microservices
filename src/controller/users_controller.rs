use actix_web::{get, post, put, delete, web, http::StatusCode, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use crate::entity::user_entity::User;
use crate::entity::user_entity::NewUser;
use crate::service::user_service::UserService;
use crate::entity::user_entity::UpdateUser;
use crate::util::responses::ResponseBuilder;
use validator::Validate;
use validator::ValidationErrors;

// Tipo del pool de conexión a la base de datos
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn list_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Llamar al servicio para obtener los usuarios
    match UserService::get_all_users(&mut *conn) {
        
        Ok(users) => HttpResponse::build(StatusCode::OK)
        .json(ResponseBuilder::<Vec<User>>::new(200)
        .message(format!("Success")).result(users)
        .build()), // Devolver la lista de usuarios en formato JSON
        Err(e) => {
            eprintln!("Error creating user: {:?}", e); // Imprimir el error en consola
            HttpResponse::InternalServerError().body(format!("Error creating user: {}", e))
        }
    }
}

#[get("/user/{user_id}")]
async fn get_user_by_id(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match UserService::get_user_by_id(&mut *conn, user_id.into_inner()) {
        Ok(user) => HttpResponse::build(StatusCode::OK)
            .json(ResponseBuilder::<User>::new(200)
            .message(format!("Success")).result(user)
            .build()),
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            HttpResponse::InternalServerError().body("Error updating user")
        }
    }
}


#[post("/user")]
async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let user = user.into_inner();

    // Validar los datos antes de intentar crear el usuario
    if let Err(validation_errors) = user.validate() {
        eprintln!("Validation error: {:?}", validation_errors);
        return HttpResponse::BadRequest().json(ResponseBuilder::<Option<ValidationErrors>>::new(400)
            .message("Validation failed".to_string())
            .result(Some(validation_errors))
            .build());
    }

    let mut conn = pool.get().expect("Failed to get DB connection");

    match UserService::create_user(&mut *conn, &user) {
        Ok(created_user) => HttpResponse::build(StatusCode::OK)
            .json(ResponseBuilder::<Option<User>>::new(200)
            .message("Success".to_string())
            .result(Some(created_user))
            .build()),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error creating user: {}", e))
        }
    }
}


#[put("/user/{user_id}")]
async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    updated_user: web::Json<UpdateUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match UserService::update_user(&mut *conn, user_id.into_inner(), &updated_user.into_inner()) {
        Ok(user) => HttpResponse::build(StatusCode::OK)
            .json(ResponseBuilder::<User>::new(201)
            .message(format!("Success")).result(user)
            .build()),
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            HttpResponse::InternalServerError().body("Error updating user")
        }
    }
}

#[delete("/user/{user_id}")]
async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let user_id = user_id.into_inner(); // Capturar el valor antes del match

    match UserService::delete_user(&mut *conn, user_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(diesel::result::Error::NotFound) => HttpResponse::build(StatusCode::NOT_FOUND)
        .json(ResponseBuilder::<Vec<String>>::new(404)
        .message(format!("User with ID {} not found", user_id))
        .build()),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

// Función para inicializar las rutas
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
        .service(create_user)
        .service(update_user)
        .service(get_user_by_id)
        .service(delete_user);
}