// @generated automatically by Diesel CLI.

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
