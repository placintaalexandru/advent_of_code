use crate::day19::blueprint::Cost;
use crate::day19::resource::Resource;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub(crate) type BagInventory = Cost;

#[derive(Debug)]
pub(crate) struct Bag {
    pub(crate) resources: BagInventory,
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            resources: HashMap::from_iter(Resource::iter().map(|resource| (resource, 0))),
        }
    }
}
