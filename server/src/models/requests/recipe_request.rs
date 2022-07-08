use serde::{Deserialize, Serialize};

// TODO: probably remove this and all like it
#[derive(Serialize, Deserialize)]
pub struct RecipeRequest {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub directions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRecipeRequest {
    pub name: String,
    pub description: Option<String>,
    pub directions: Option<String>,
}
