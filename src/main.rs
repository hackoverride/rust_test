use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;

#[get("/status")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./public").index_file("index.html").prefer_utf8(true))
            .service(hello)
            .service(echo)
            .route("/api/hey", web::get().to(manual_hello))
    })
    .bind(format!("0.0.0.0:{}", std::env::var("PORT").unwrap_or_else(|_| String::from("8080"))))?
    .run()
    .await
}