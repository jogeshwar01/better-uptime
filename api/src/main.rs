use crate::types::{
    request::{CreateWebsiteInput, GetWebsiteInput},
    response::{CreateWebsiteOutput, GetWebsiteOutput},
};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::StatusCode,
    post,
    web::{self},
};
use store::store::Store;

pub mod types;

#[get("/website/{website_id}")]
async fn get_website(path: web::Path<GetWebsiteInput>) -> impl Responder {
    let input = path.into_inner(); // Only call once

    let mut s = Store::new().unwrap();
    let website = s.get_website(input.website_id.to_string()).unwrap();

    web::Json(GetWebsiteOutput { url: website.url })
}

#[post("/website")]
async fn create_website_fn(body: web::Json<CreateWebsiteInput>) -> impl Responder {
    let input = body.into_inner();

    // Empty URL -> return custom 422 status
    if input.url.trim().is_empty() {
        return HttpResponse::build(StatusCode::from_u16(422).unwrap())
            .body("URL cannot be empty - custom 422 Unprocessable Entity");
    }

    let mut s = Store::new().unwrap();
    let website = s
        .create_website(input.user_id.to_string(), input.url)
        .unwrap();

    HttpResponse::Ok().json(CreateWebsiteOutput { id: website.id })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_website).service(create_website_fn))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
