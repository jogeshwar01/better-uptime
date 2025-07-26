use actix_web::middleware::Logger;
use actix_web::{
    App, HttpServer,
    web::{self},
};
use env_logger::Env;
use std::sync::{Arc, Mutex};
use store::store::Store;

pub mod routes;
pub mod types;

pub struct AppState {
    pub store: Arc<Mutex<Store>>,
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Website routes
        .route(
            "/website/{website_id}",
            web::get().to(routes::website::get_website),
        )
        .route(
            "/website",
            web::post().to(routes::website::create_website_fn),
        )
        // User routes
        .route("/signup", web::post().to(routes::user::signup))
        .route("/signin", web::post().to(routes::user::signin));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store = Arc::new(Mutex::new(Store::new().unwrap()));
    let state = web::Data::new(AppState { store });
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
