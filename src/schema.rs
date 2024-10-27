// @generated automatically by Diesel CLI.

diesel::table! {
    spells (id) {
        id -> Int4,
        name -> Varchar,
        level -> Int4,
        time -> Varchar,
        school -> Varchar,
        concentration -> Bool,
        range -> Varchar,
        duration -> Varchar,
    }
}
