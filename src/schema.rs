// @generated automatically by Diesel CLI.

diesel::table! {
    spells (id) {
        id -> Int4,
        name -> Varchar,
        level -> Varchar,
        time -> Varchar,
        school -> Varchar,
        concentration -> Bool,
        range -> Varchar,
        duration -> Varchar,
    }
}
