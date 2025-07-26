use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteInput {
    pub website_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String,
}
