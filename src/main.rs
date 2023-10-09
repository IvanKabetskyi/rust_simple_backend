mod application;
mod domain;
mod infrastructure;

use actix_web::{App, HttpServer};

use application::controller::user::{create_user, get_user, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user)
            .service(create_user)
            .service(update_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
