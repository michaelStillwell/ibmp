use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeasurementResponse {
    id: i32,
    name: String,
}
