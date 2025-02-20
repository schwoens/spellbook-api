use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper,
};

use crate::{
    models::users::{NewUser, User},
    schema::users::{self, key_hash},
};

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    users::table.select(User::as_select()).load(conn)
}

pub fn get_user_by_key_hash(
    conn: &mut PgConnection,
    hash: String,
) -> Result<User, diesel::result::Error> {
    users::table
        .select(User::as_select())
        .filter(key_hash.eq(hash))
        .first(conn)
}

pub fn insert_user(
    conn: &mut PgConnection,
    new_user: NewUser,
) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn)
}
