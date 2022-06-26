use crate::models::db::{NewRecipe, Recipe};
use crate::schema::ingredients::dsl::*;
use crate::schema::recipes::dsl::*;
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

pub async fn get_all(conn: &PgConnection) -> Option<Vec<Recipe>> {
    match recipes.load::<Recipe>(&*conn).optional() {
        Ok(_recipes) => _recipes,
        Err(_) => None,
    }
}

pub async fn get_by_id(_id: i32, conn: &PgConnection) -> Option<Recipe> {
    match recipes
        .filter(crate::schema::recipes::id.eq(_id))
        .first(&*conn)
        .optional()
    {
        Ok(_recipe) => _recipe,
        Err(_) => None,
    }
}

pub async fn create(_recipe: NewRecipe, conn: &PgConnection) -> Option<Recipe> {
    match diesel::insert_into(recipes)
        .values(_recipe)
        .get_result(&*conn)
        .optional()
    {
        Ok(new_recipe) => new_recipe,
        Err(_) => None,
    }
}

pub async fn update(_id: i32, _recipe: NewRecipe, conn: &PgConnection) -> Option<Recipe> {
    match diesel::update(recipes)
        .filter(crate::schema::recipes::id.eq(_id))
        .set(_recipe)
        .get_result(&*conn)
        .optional()
    {
        Ok(updated_recipe) => updated_recipe,
        Err(_) => None,
    }
}

pub async fn delete(_id: i32, conn: &PgConnection) -> bool {
    match diesel::delete(recipes.filter(crate::schema::recipes::id.eq(_id)))
        .execute(&*conn)
        .optional()
    {
        Ok(rows) => rows.unwrap() > 0,
        Err(_) => false,
    }
}
