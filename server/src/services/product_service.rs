use crate::models::db::NewProduct;
use crate::models::requests::NewProductRequest;
use crate::models::responses::ProductResponse;
use crate::repositories::product_repository;
use crate::DbPool;
use actix_web::{web, HttpResponse, Responder};

//
// NOTE: this endpoint will be protected under admin privilege for now
// don't want people mass inputting dupes and misspells
//

// TODO: implement user based products

#[get("/products")]
async fn get_products(pool: web::Data<DbPool>) -> impl Responder {
    match product_repository::get_all(&pool.get().unwrap()).await {
        Some(products) => HttpResponse::Ok().json(
            products
                .iter()
                .map(|p| ProductResponse::from(p))
                .collect::<Vec<ProductResponse>>(),
        ),
        None => HttpResponse::Ok().json(vec![] as Vec<ProductResponse>),
    }
}

#[get("/product/{id}")]
async fn get_product_by_id(pool: web::Data<DbPool>, id: web::Path<(i32,)>) -> impl Responder {
    match product_repository::get_by_id(id.into_inner().0, &pool.get().unwrap()).await {
        Some(product) => HttpResponse::Ok().json(ProductResponse::from(&product)),
        None => HttpResponse::NoContent().json(()),
    }
}

#[post("/products")]
async fn create_product(
    pool: web::Data<DbPool>,
    product_request: web::Json<NewProductRequest>,
) -> impl Responder {
    match product_repository::create(
        NewProduct::from(product_request.into_inner()),
        &pool.get().unwrap(),
    )
    .await
    {
        Some(new_product) => HttpResponse::Ok().json(ProductResponse::from(&new_product)),
        None => HttpResponse::NoContent().json(()),
    }
}

#[put("/product/{id}")]
async fn update_product(
    pool: web::Data<DbPool>,
    id: web::Path<(i32,)>,
    product_request: web::Json<NewProductRequest>,
) -> impl Responder {
    match product_repository::update(
        id.into_inner().0,
        NewProduct::from(product_request.into_inner()),
        &pool.get().unwrap(),
    )
    .await
    {
        Some(product) => HttpResponse::Ok().json(ProductResponse::from(&product)),
        None => HttpResponse::NoContent().json(()),
    }
}

#[delete("/product/{id}")]
async fn delete_product(pool: web::Data<DbPool>, id: web::Path<(i32,)>) -> impl Responder {
    match product_repository::delete(id.into_inner().0, &pool.get().unwrap()).await {
        true => HttpResponse::Ok().json(()),
        false => HttpResponse::NoContent().json(()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_products)
        .service(get_product_by_id)
        .service(create_product)
        .service(update_product)
        .service(delete_product);
}
