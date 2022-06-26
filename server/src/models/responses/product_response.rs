use crate::models::db::Product;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
}

impl From<&Product> for ProductResponse {
    fn from(product: &Product) -> Self {
        Self {
            id: product.id,
            name: product.name.clone(),
        }
    }
}
