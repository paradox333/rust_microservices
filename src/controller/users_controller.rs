use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// Function to initialize routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(echo)
        .route("/users", web::get().to(manual_hello));
}
