// @generated automatically by Diesel CLI.

diesel::table! {
    contents (id) {
        id -> Int4,
        author -> Varchar,
        title -> Varchar,
        link -> Varchar,
        description -> Varchar,
        published -> Bool,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        avatar -> Nullable<Varchar>,
        publickey -> Varchar,
        privatekey -> Varchar,
        u_id -> Int4,
    }
}

diesel::joinable!(contents -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contents,
    users,
);
