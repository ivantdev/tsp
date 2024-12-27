// @generated automatically by Diesel CLI.

diesel::table! {
    trips (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 250]
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
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 32]
        salt -> Varchar,
        #[max_length = 128]
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
