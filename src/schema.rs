// @generated automatically by Diesel CLI.

diesel::table! {
    trips (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Nullable<Varchar>,
        locations -> Json,
        path -> Json,
        distance -> Float8,
        completed -> Bool,
        created_on -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        username -> Varchar,
        email -> Varchar,
        salt -> Varchar,
        password -> Varchar,
        picture -> Nullable<Text>,
        created_on -> Timestamp,
        staff -> Bool,
        admin -> Bool,
    }
}

diesel::joinable!(trips -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    trips,
    users,
);
