// @generated automatically by Diesel CLI.

diesel::table! {
    spells (id) {
        id -> Int4,
        name -> Varchar,
        level -> Varchar,
        casting_time -> Varchar,
        magic_school -> Varchar,
        concentration -> Bool,
        range -> Varchar,
        duration -> Varchar,
        description -> Text,
        user_id -> Int4,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        key_hash -> Varchar,
    }
}

diesel::joinable!(spells -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    spells,
    users,
);
