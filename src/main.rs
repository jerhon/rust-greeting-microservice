use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Greeting {
    greeting: String,
}

async fn greeting() -> impl Responder {
    let ret = Greeting { greeting: "Hello, world!".to_string() };
    web::Json(ret)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .route("/greeting", web::get().to(greeting))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
