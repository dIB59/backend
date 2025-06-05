use actix_web::{App, HttpServer, web, HttpRequest, HttpResponse};
use actix_web::web::Payload;
use actix_web_actors::ws;
use ws_health_handler::HeathSocket;


mod ws_health_handler;

async fn health_handler(req: HttpRequest, stream: Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(HeathSocket {}, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ws", web::get().to(health_handler))
            .service(web::resource("/").to(|| async {
                HttpResponse::Ok().body("Welcome to the WebSocket server!")
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
