use crate::models::db::{NewProduct, Product};
use crate::schema::products::dsl::*;
use diesel::insert_into;
use diesel::prelude::*;

pub async fn get_all(conn: &PgConnection) -> Option<Vec<Product>> {
    match products.load::<Product>(&*conn).optional() {
        Ok(_products) => _products,
        Err(_) => None,
    }
}

pub async fn get_by_id(_id: i32, conn: &PgConnection) -> Option<Product> {
    match products.filter(id.eq(_id)).first(&*conn).optional() {
        Ok(product) => product,
        Err(_) => None,
    }
}

pub async fn create(product: NewProduct, conn: &PgConnection) -> Option<Product> {
    match insert_into(products)
        .values(product)
        .get_result(&*conn)
        .optional()
    {
        Ok(new_product) => new_product,
        Err(_) => None,
    }
}

// NOTE: this could probably just be a bool, don't need the whole object
pub async fn update(_id: i32, product: NewProduct, conn: &PgConnection) -> Option<Product> {
    match diesel::update(products)
        .filter(id.eq(_id))
        .set(product)
        .get_result(&*conn)
        .optional()
    {
        Ok(updated_product) => updated_product,
        Err(_) => None,
    }
}

pub async fn delete(_id: i32, conn: &PgConnection) -> bool {
    match diesel::delete(products.filter(id.eq(_id)))
        .execute(&*conn)
        .optional()
    {
        Ok(rows) => rows.unwrap() > 0,
        Err(_) => false,
    }
}
