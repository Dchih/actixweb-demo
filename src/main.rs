use actix_web::{web, App, HttpServer, HttpResponse};

mod api;
use api::greety;
use api::get_user_name;
use api::login;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greety)
            .service(login)
            .service(get_user_name)
            .route("/", web::get().to(|| async {HttpResponse::Ok().body("Hi")}))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
