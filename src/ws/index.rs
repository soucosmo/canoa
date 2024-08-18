use actix_web_actors::ws;
use super::WsServer;
use actix_web::{
    HttpRequest,
    HttpResponse,
    Error,
    web::Payload,
};

pub async fn index(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsServer {}, &req, stream);
    println!("{:?}", resp);
    resp
}
