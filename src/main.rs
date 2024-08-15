use actix_web::{App, HttpServer, web::get};

mod http;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", get().to(http::HttpServer::hello))
    })
    .bind(("127.0.0.1", 9797))?
    .run()
    .await
}
