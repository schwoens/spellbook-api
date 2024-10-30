use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    models::{NewSpell, Spell},
    schema::spells::{self, name},
};

pub fn get_spells(conn: &mut PgConnection) -> Result<Vec<Spell>, diesel::result::Error> {
    spells::table.select(Spell::as_select()).load(conn)
}

pub fn get_spell(
    conn: &mut PgConnection,
    spell_name: &str,
) -> Result<Spell, diesel::result::Error> {
    spells::table
        .select(Spell::as_select())
        .filter(name.eq(spell_name))
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

pub type UpdatedSpell<'a> = NewSpell<'a>;

pub fn update_spell(
    conn: &mut PgConnection,
    spell_name: &str,
    updated_spell: UpdatedSpell,
) -> Result<Spell, diesel::result::Error> {
    diesel::update(spells::table)
        .filter(name.eq(spell_name))
        .set(updated_spell)
        .returning(Spell::as_returning())
        .get_result(conn)
}

pub fn delete_spell(
    conn: &mut PgConnection,
    spell_name: &str,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(spells::table)
        .filter(name.eq(spell_name))
        .execute(conn)
}
