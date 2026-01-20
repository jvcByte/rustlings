mod api;
mod deps;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use api::task::get_task;
use deps::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).app_data().service(get_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
