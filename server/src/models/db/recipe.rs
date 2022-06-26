use crate::models::requests::NewRecipeRequest;
use crate::schema::recipes;

#[derive(Debug, Queryable, Insertable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub directions: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "recipes"]
pub struct NewRecipe {
    pub name: String,
    pub description: Option<String>,
    pub directions: Option<String>,
}

impl From<&NewRecipeRequest> for NewRecipe {
    fn from(recipe: &NewRecipeRequest) -> Self {
        Self {
            name: recipe.name.clone(),
            description: recipe.description.clone(),
            directions: recipe.directions.clone(),
        }
    }
}
