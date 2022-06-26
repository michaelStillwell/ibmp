use crate::models::responses::{MeasurementResponse, ProductResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IngredientResponse {
    id: i32,
    recipe_id: i32,
    product: ProductResponse,
    amount: f32,
    measurement: MeasurementResponse,
}
