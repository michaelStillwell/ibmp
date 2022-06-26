use crate::models::db::Recipe;
use crate::models::responses::IngredientResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RecipeResponse {
    id: i32,
    name: String,
    description: Option<String>,
    directions: Option<String>,
    ingredients: Vec<IngredientResponse>,
}

impl From<&Recipe> for RecipeResponse {
    fn from(val: &Recipe) -> Self {
        Self {
            id: val.id,
            name: val.name.to_string(),
            description: match &val.description {
                Some(desc) => Some(desc.clone()),
                None => None,
            },
            directions: match &val.directions {
                Some(dir) => Some(dir.clone()),
                None => None,
            },
            ingredients: vec![],
        }
    }
}
