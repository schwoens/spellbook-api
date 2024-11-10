use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    models::users::{NewUser, User},
    schema::users,
};

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    users::table.select(User::as_select()).load(conn)
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
