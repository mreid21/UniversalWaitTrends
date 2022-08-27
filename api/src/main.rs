use actix_web::{App, HttpServer};
use handlers::{get_wait_times, get_wait_time_for};

mod handlers;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_wait_times)
            .service(get_wait_time_for)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}