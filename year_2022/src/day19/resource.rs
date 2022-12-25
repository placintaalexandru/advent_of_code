use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Hash, EnumIter, PartialOrd, Ord)]
pub(crate) enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
