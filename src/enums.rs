use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames)]
pub enum MagicSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

