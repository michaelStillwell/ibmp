table! {
    ingredients (id) {
        id -> Int4,
        recipe_id -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        amount -> Nullable<Float8>,
        measurement_id -> Nullable<Int4>,
    }
}

table! {
    measurements (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        directions -> Nullable<Varchar>,
    }
}

joinable!(ingredients -> measurements (measurement_id));
joinable!(ingredients -> products (product_id));
joinable!(ingredients -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    measurements,
    products,
    recipes,
);
