use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteInput {
    pub website_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub user_id: Uuid,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
}
