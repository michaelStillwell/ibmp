table! {
    ingredients (id) {
        id -> Int4,
        recipe -> Nullable<Int4>,
        product -> Nullable<Int4>,
        amount -> Nullable<Float8>,
        measurement -> Nullable<Int4>,
    }
}

table! {
    measurements (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
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

joinable!(ingredients -> measurements (measurement));
joinable!(ingredients -> products (product));
joinable!(ingredients -> recipes (recipe));

allow_tables_to_appear_in_same_query!(ingredients, measurements, products, recipes,);
