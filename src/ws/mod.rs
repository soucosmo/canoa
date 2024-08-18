use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
mod index;


enum Op {
    PodRun {
        name: String
    }
}
pub use index::index;

pub struct WsServer;

impl Actor for WsServer {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("piiiiiiiiiing");
                ctx.pong(&msg)
            },
            Ok(ws::Message::Text(text)) => {
                let m = text.trim();

                println!("text [{}]", m);

                ctx.text(text)
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Binary");
                ctx.binary(bin)
            },
            _ => (),
        }
    }
}
