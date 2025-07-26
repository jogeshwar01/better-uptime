use crate::AppState;
use crate::types::{
    request::{CreateUserInput, GetUserInput},
    response::{CreateUserOutput, SigninOutput},
};
use actix_web::{HttpResponse, Responder, web};

pub async fn signup(body: web::Json<CreateUserInput>, data: web::Data<AppState>) -> impl Responder {
    let input = body.into_inner();

    if input.username.trim().is_empty() || input.password.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Username and password cannot be empty");
    }

    let mut store = data.store.lock().unwrap();

    match store.sign_up(input.username, input.password) {
        Ok(user_id) => HttpResponse::Ok().json(CreateUserOutput { id: user_id }),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().body("Could not create user")
        }
    }
}

pub async fn signin(body: web::Json<GetUserInput>, data: web::Data<AppState>) -> impl Responder {
    let input = body.into_inner();

    if input.username.trim().is_empty() || input.password.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Username and password cannot be empty");
    }

    let mut store = data.store.lock().unwrap();

    match store.sign_in(input.username, input.password) {
        Ok(success) => {
            if success {
                // For now, returning a simple JWT placeholder
                // In a real app, you'd generate a proper JWT token
                HttpResponse::Ok().json(SigninOutput {
                    jwt: "jwt_token_placeholder".to_string(),
                })
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().body("Could not authenticate user")
        }
    }
}
