use actix_web::{get, post, web, App, middleware, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

mod server;
use self::server::MyWebSocket;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize, Deserialize)]
struct Temperature {
    temperature: f32,
}

#[get("/current_temperature")]
async fn current_temperature() -> actix_web::Result<impl Responder> {
    Ok(web::Json(Temperature { temperature: 32.0 }))
}

/// WebSocket handshake and start `MyWebSocket` actor.
#[get("/ws/random_integer")]
async fn random_integer_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let myws = MyWebSocket::new();
    let resp = ws::start(myws, &req, stream)?;
    Ok(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(current_temperature)
            .service(random_integer_ws)
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
