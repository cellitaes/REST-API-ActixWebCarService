pub mod api;

use api::car::{
    get_disselUsage,
    get_injectorFail
};

use actix_web::{HttpServer, App, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        .service(get_disselUsage)
        .service(get_injectorFail)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
