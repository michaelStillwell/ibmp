use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProductRequest {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewProductRequest {
    pub name: String,
}
