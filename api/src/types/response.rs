use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteOutput {
    pub website_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}
