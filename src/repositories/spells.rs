use diesel::{
    ExpressionMethods, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

use crate::{
    models::{
        spells::{NewSpell, Spell, UpdatedSpell},
        users::User,
    },
    requests::spells::QuerySpellRequest,
    schema::{
        spells::{
            self, casting_time, concentration, duration, level, magic_school, name, nanoid,
            published, range, user_id,
        },
        users,
    },
};

pub fn get_spells(conn: &mut PgConnection, u_id: i32) -> Result<Vec<Spell>, diesel::result::Error> {
    spells::table
        .select(Spell::as_select())
        .filter(user_id.eq(u_id))
        .load(conn)
}

pub fn get_spell_by_nanoid(
    conn: &mut PgConnection,
    u_id: i32,
    n_id: &str,
) -> Result<Spell, diesel::result::Error> {
    spells::table
        .select(Spell::as_select())
        .filter(user_id.eq(u_id))
        .filter(nanoid.eq(n_id))
        .first(conn)
}

pub fn get_spell_by_name(
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
    n_id: String,
    updated_spell: UpdatedSpell,
) -> Result<Spell, diesel::result::Error> {
    diesel::update(spells::table)
        .filter(user_id.eq(u_id))
        .filter(nanoid.eq(n_id))
        .set(updated_spell)
        .returning(Spell::as_returning())
        .get_result(conn)
}

pub fn delete_spell(
    conn: &mut PgConnection,
    u_id: i32,
    n_id: &str,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(spells::table)
        .filter(user_id.eq(u_id))
        .filter(nanoid.eq(n_id))
        .execute(conn)
}

pub fn publish_spell(
    conn: &mut PgConnection,
    u_id: i32,
    n_id: &str,
    publish: bool,
) -> Result<Spell, diesel::result::Error> {
    diesel::update(spells::table)
        .filter(user_id.eq(u_id))
        .filter(nanoid.eq(n_id))
        .set(published.eq(publish))
        .returning(Spell::as_returning())
        .get_result(conn)
}

pub fn query_spells(
    conn: &mut PgConnection,
    u_id: i32,
    query_data: QuerySpellRequest,
) -> Result<Vec<Spell>, diesel::result::Error> {
    let mut query = spells::table.into_boxed();
    query = query.filter(user_id.eq(u_id));
    if let Some(query_name) = query_data.name {
        query = query.filter(name.ilike(format!("%{}%", query_name)))
    }
    if let Some(query_level) = query_data.level {
        query = query.filter(level.ilike(format!("%{}%", query_level)))
    }
    if let Some(query_casting_time) = query_data.casting_time {
        query = query.filter(casting_time.ilike(format!("%{}%", query_casting_time)))
    }
    if let Some(query_magic_school) = query_data.magic_school {
        query = query.filter(magic_school.ilike(format!("%{}%", query_magic_school)))
    }
    if let Some(query_concentration) = query_data.concentration {
        query = query.filter(concentration.eq(query_concentration))
    }
    if let Some(query_range) = query_data.range {
        query = query.filter(range.ilike(format!("%{}%", query_range)))
    }
    if let Some(query_duration) = query_data.duration {
        query = query.filter(duration.ilike(format!("%{}%", query_duration)))
    }
    query.load(conn)
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
    n_id: &str,
) -> Result<bool, diesel::result::Error> {
    spells::table
        .select(Spell::as_select())
        .filter(user_id.eq(u_id))
        .filter(nanoid.eq(n_id))
        .get_result(conn)
        .map(|s| s.published)
}
