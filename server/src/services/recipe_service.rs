use crate::models::db::NewRecipe;
use crate::models::requests::NewRecipeRequest;
use crate::models::responses::RecipeResponse;
use crate::repositories::recipe_repository;
use crate::DbPool;
use actix_web::{web, HttpResponse, Responder};

//
// NOTE: this endpoint will be protected under admin privilege for now
// don't want people mass inputting dupes and misspells
//

// TODO: implement user based recipes

#[get("/recipes")]
async fn get_all_recipes(pool: web::Data<DbPool>) -> impl Responder {
    match recipe_repository::get_all(&pool.get().unwrap()).await {
        Some(recipes) => HttpResponse::Ok().json(
            recipes
                .iter()
                .map(|r| RecipeResponse::from(r))
                .collect::<Vec<RecipeResponse>>(),
        ),
        None => HttpResponse::Ok().json(vec![] as Vec<RecipeResponse>),
    }
}

#[get("/recipe/{id}")]
async fn get_recipe(pool: web::Data<DbPool>, id: web::Path<(i32,)>) -> impl Responder {
    match recipe_repository::get_by_id(id.into_inner().0, &pool.get().unwrap()).await {
        Some(recipe) => HttpResponse::Ok().json(RecipeResponse::from(&recipe)),
        None => HttpResponse::NoContent().json(()),
    }
}

#[post("/recipes")]
async fn create_recipe(
    pool: web::Data<DbPool>,
    recipe_request: web::Json<NewRecipeRequest>,
) -> impl Responder {
    match recipe_repository::create(
        NewRecipe::from(&recipe_request.into_inner()),
        &pool.get().unwrap(),
    )
    .await
    {
        Some(recipe) => HttpResponse::Ok().json(RecipeResponse::from(&recipe)),
        None => HttpResponse::NoContent().json(()),
    }
}

#[put("/recipe/{id}")]
async fn update_recipe(
    pool: web::Data<DbPool>,
    id: web::Path<(i32,)>,
    recipe_request: web::Json<NewRecipeRequest>,
) -> impl Responder {
    match recipe_repository::update(
        id.into_inner().0,
        NewRecipe::from(&recipe_request.into_inner()),
        &pool.get().unwrap(),
    )
    .await
    {
        Some(recipe) => HttpResponse::Ok().json(RecipeResponse::from(&recipe)),
        None => HttpResponse::NoContent().finish(),
    }
}

#[delete("/recipe/{id}")]
async fn delete_recipe(pool: web::Data<DbPool>, id: web::Path<(i32,)>) -> impl Responder {
    match recipe_repository::delete(id.into_inner().0, &pool.get().unwrap()).await {
        true => HttpResponse::Ok().finish(),
        false => HttpResponse::NoContent().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_recipes)
        .service(get_recipe)
        .service(create_recipe)
        .service(update_recipe)
        .service(delete_recipe);
}
