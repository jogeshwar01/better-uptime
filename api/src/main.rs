use crate::types::{
    request::{CreateWebsiteInput, GetWebsiteInput},
    response::{CreateWebsiteOutput, GetWebsiteOutput},
};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, post,
    web::{self},
};
use std::sync::{Arc, Mutex};
use store::store::Store;

pub mod types;
pub struct AppState {
    pub store: Arc<Mutex<Store>>,
}

#[get("/website/{website_id}")]
async fn get_website(
    path: web::Path<GetWebsiteInput>,
    data: web::Data<AppState>,
) -> impl Responder {
    let input = path.into_inner();
    let mut store = data.store.lock().unwrap();

    match store.get_website(input.website_id.to_string()) {
        Ok(website) => HttpResponse::Ok().json(GetWebsiteOutput { url: website.url }),
        Err(_) => HttpResponse::NotFound().body("Website not found"),
    }
}

#[post("/website")]
async fn create_website_fn(
    body: web::Json<CreateWebsiteInput>,
    data: web::Data<AppState>,
) -> impl Responder {
    let input = body.into_inner();

    if input.url.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("URL cannot be empty");
    }

    let mut store = data.store.lock().unwrap();

    match store.create_website(input.user_id.to_string(), input.url) {
        Ok(website) => HttpResponse::Ok().json(CreateWebsiteOutput { id: website.id }),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().body("Could not create website")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store = Arc::new(Mutex::new(Store::new().unwrap()));
    let state = web::Data::new(AppState { store });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_website)
            .service(create_website_fn)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
