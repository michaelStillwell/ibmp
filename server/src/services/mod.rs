use actix_web::web;
mod product_service;
mod recipe_service;

pub fn init(cfg: &mut web::ServiceConfig) {
    product_service::init(cfg);
    recipe_service::init(cfg);
}
