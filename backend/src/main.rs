mod config;
mod handlers;
mod models;
mod testing;
mod util;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = crate::config::AppConfig::new();
    let server_addr = config.server_addr.clone();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.pg_conn_string)
        .await
        .unwrap();
    let server = HttpServer::new(move || {
        let person_scope = web::scope("/person")
            .service(handlers::person::get_persons)
            .wrap(HttpAuthentication::bearer(util::auth_validator));
        let auth_scope = web::scope("/auth").service(handlers::auth::login);
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::monitoring::heartbeat)
            .service(testing::test)
            .service(person_scope)
            .service(auth_scope)
    })
    .bind(server_addr)?
    .run();
    server.await
}
