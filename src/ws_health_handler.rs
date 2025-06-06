use std::sync::Mutex;

use actix_web::{web::{self, Data, Payload}, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler};


pub struct ConnectionState {
    pub counter: Mutex<usize>,
}

pub struct HeathSocket {
    state: web::Data<ConnectionState>,
}

impl HeathSocket {
    pub fn new(state: web::Data<ConnectionState>) -> Self {
        Self { state }
    }
}

impl Actor for HeathSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut counter = self.state.counter.lock().unwrap();
        *counter += 1;
        ctx.text(format!("Connected: {:?}", counter));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for HeathSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let counter = self.state.get_ref().counter.lock().unwrap();
                ctx.text(format!("Echo: {}, {:?}", text, counter));
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(close)) => {
                let mut counter = self.state.counter.lock().unwrap();
                *counter -= 1;
                ctx.text(format!("Disconnected: {:?}", counter));
                ctx.close(close);
            }
            _ => (),
        }
    }
}

pub async fn health_handler(req: HttpRequest, stream: Payload, state: Data<ConnectionState>) -> Result<HttpResponse, actix_web::Error> {
    ws::start(HeathSocket::new(state), &req, stream)
}
