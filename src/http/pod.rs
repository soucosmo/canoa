use super::HttpServer;
use actix_web::Responder;

impl HttpServer {
    pub async fn pod_run() -> impl Responder {
        "Running pod"
    }
}
