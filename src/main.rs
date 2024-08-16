use actix_web::{App, HttpServer, web::{get, post, scope}};

mod http;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", get().to(http::HttpServer::hello))
            .service(
                scope("/pod")
                    .route("", post().to(http::HttpServer::pod_run))
            )
    })
    .bind(("127.0.0.1", 9797))?
    .run()
    .await
}
