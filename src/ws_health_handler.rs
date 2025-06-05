use actix_web::web::Data;
use actix_web_actors::ws;
use actix::{Actor, StreamHandler};


pub struct HeathSocket{
    channel: String,
    state: Data<i32>,
}

impl Actor for HeathSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Client connected");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for HeathSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Got: {}", text);
                ctx.text(format!("Echo: {}", text));
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(close)) => {
                println!("Client disconnected: {:?}", close);
                ctx.close(close);
            }
            _ => (),
        }
    }
}
