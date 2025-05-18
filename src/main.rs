use actix_web::{get, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use awc::{Client, ClientResponse};
use askama::Template;
mod polymer;
mod innertube;

#[get("/")]
async fn home() -> impl Responder {
    let tmpl = polymer::CoreTemplate { config: "{\"test\":\"test\"}", initial_data: "shit" };
    HttpResponse::Ok().body(tmpl.render().unwrap())
}

async fn default(req: HttpRequest) -> impl Responder {
    let mut headers = req.headers().iter();
    let mut client = Client::default();
    let mut url = innertube::HOST.to_owned();
    url.push_str(&req.uri().to_string());

    let mut request = client.request(req.method().clone(), url);
    for header in headers {
        request = request.insert_header(header);
    }

    request.send()
        .map_err(|_| ())
        .and_then(|response| {
        
        });

    HttpResponse::Ok().body(req.uri().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .default_service(web::get().to(default))
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}