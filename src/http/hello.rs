
use actix_web::Responder;
use super::HttpServer;


impl HttpServer {
    pub async fn hello() -> impl Responder {
        "Hello, Canoa is working!"
    }
}
