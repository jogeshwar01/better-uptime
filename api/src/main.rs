use crate::types::{
    request::{CreateWebsiteInput, GetWebsiteInput},
    response::{CreateWebsiteOutput, GetWebsiteOutput},
};
use actix_web::{
    get, http::StatusCode, post, web::{self}, App, HttpResponse, HttpServer, Responder
};
pub mod types;

#[get("/website/{website_id}")]
async fn get_website(path: web::Path<GetWebsiteInput>) -> impl Responder {
    let input = path.into_inner(); // Only call once
    let response = GetWebsiteOutput {
        website_id: input.website_id,
    };

    HttpResponse::Ok().json(response)
}

#[post("/website")]
async fn create_website_fn(body: web::Json<CreateWebsiteInput>) -> impl Responder {
    let input = body.into_inner();

    // Empty URL -> return custom 422 status
    if input.url.trim().is_empty() {
        return HttpResponse::build(StatusCode::from_u16(422).unwrap())
            .body("URL cannot be empty - custom 422 Unprocessable Entity");
    }

    let body = CreateWebsiteOutput {
        id: String::from("website_id"),
    };

    HttpResponse::Ok().json(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_website).service(create_website_fn))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
