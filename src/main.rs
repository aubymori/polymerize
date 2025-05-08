use actix_web::{get, web, App, HttpResponse, HttpRequest, HttpServer, Responder}; 
use askama::Template;
mod polymer;

#[get("/")]
async fn home() -> impl Responder {
    let tmpl = polymer::CoreTemplate { config: "{\"test\":\"test\"}", initial_data: "shit" };
    HttpResponse::Ok().body(tmpl.render().unwrap())
}

async fn default(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("default")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .default_service(web::get().to(default))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}