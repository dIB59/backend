use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;


mod ws_health_handler;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(ws_health_handler::ConnectionState {
        counter: Mutex::new(0),
    });
    
    HttpServer::new(move || {
        App::new()
        .app_data(shared_data.clone())
            .route("/ws/health", web::get().to(ws_health_handler::health_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
