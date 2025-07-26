use crate::AppState;
use crate::types::{
    request::{CreateWebsiteInput, GetWebsiteInput},
    response::{CreateWebsiteOutput, GetWebsiteOutput},
};
use actix_web::{HttpResponse, Responder, web};

pub async fn get_website(
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

pub async fn create_website_fn(
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
