use diesel::{query_dsl::methods::SelectDsl, PgConnection, RunQueryDsl, SelectableHelper};

use crate::{
    models::{NewSpell, Spell},
    schema::spells,
};

pub fn get_spells(conn: &mut PgConnection) -> Result<Vec<Spell>, diesel::result::Error> {
    spells::table.select(Spell::as_select()).load(conn)
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
