use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health() -> impl Responder{
    HttpResponse::Ok()
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
