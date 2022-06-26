use crate::models::requests::NewProductRequest;
use crate::schema::products;

#[derive(Debug, Queryable, PartialEq, Insertable, AsChangeset)]
pub struct Product {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
}

impl From<NewProductRequest> for NewProduct {
    fn from(product: NewProductRequest) -> Self {
        Self {
            name: product.name.clone(),
        }
    }
}
