use diesel::{
    ExpressionMethods, JoinOnDsl, NullableExpressionMethods, PgConnection, PgTextExpressionMethods,
    QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::{
    models::{
        spells::{NewSpell, Spell, UpdatedSpell},
        users::User,
    },
    schema::{
        spells::{self, name, published, user_id},
        users,
    },
};

pub fn get_spells(conn: &mut PgConnection, u_id: i32) -> Result<Vec<Spell>, diesel::result::Error> {
    spells::table
        .select(Spell::as_select())
        .filter(user_id.eq(u_id))
        .load(conn)
}

pub fn get_spell(
    conn: &mut PgConnection,
    u_id: i32,
    spell_name: &str,
) -> Result<Spell, diesel::result::Error> {
    spells::table
        .select(Spell::as_select())
        .filter(user_id.eq(u_id))
        .filter(name.ilike(spell_name))
        .first(conn)
}

pub fn insert_spell(
    conn: &mut PgConnection,
    new_spell: NewSpell,
) -> Result<Spell, diesel::result::Error> {
    diesel::insert_into(spells::table)
        .values(new_spell)
        .returning(Spell::as_returning())
        .get_result(conn)
}

pub fn update_spell(
    conn: &mut PgConnection,
    u_id: i32,
    spell_name: &str,
    updated_spell: UpdatedSpell,
) -> Result<Spell, diesel::result::Error> {
    diesel::update(spells::table)
        .filter(user_id.eq(u_id))
        .filter(name.eq(spell_name))
        .set(updated_spell)
        .returning(Spell::as_returning())
        .get_result(conn)
}

pub fn delete_spell(
    conn: &mut PgConnection,
    u_id: i32,
    spell_name: &str,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(spells::table)
        .filter(user_id.eq(u_id))
        .filter(name.eq(spell_name))
        .execute(conn)
}

pub fn publish_spell(
    conn: &mut PgConnection,
    u_id: i32,
    spell_name: &str,
    publish: bool,
) -> Result<Spell, diesel::result::Error> {
    diesel::update(spells::table)
        .filter(user_id.eq(u_id))
        .filter(name.ilike(spell_name))
        .set(published.eq(publish))
        .returning(Spell::as_returning())
        .get_result(conn)
}

pub fn query_public_spells(
    conn: &mut PgConnection,
    keyword: &str,
) -> Result<Vec<(Spell, User)>, diesel::result::Error> {
    spells::table
        .inner_join(users::table)
        .filter(name.ilike(format!("%{}%", keyword)))
        .filter(published.eq(true))
        .select((Spell::as_select(), User::as_select()))
        .load::<(Spell, User)>(conn)
}

pub fn is_published(
    conn: &mut PgConnection,
    u_id: i32,
    spell_name: &str,
) -> Result<bool, diesel::result::Error> {
    spells::table
        .select(Spell::as_select())
        .filter(user_id.eq(u_id))
        .filter(name.ilike(spell_name))
        .get_result(conn)
        .map(|s| s.published)
}
