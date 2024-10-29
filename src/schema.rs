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
    }
}
