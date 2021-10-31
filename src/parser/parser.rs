use std::collections::BTreeMap;
use super::grouper::Object;

pub struct Parser {
    objects: Vec<Object>,
    object_table: BTreeMap<u64, Object>
}

impl Parser {
    fn new(objects: Vec<Object>) -> Self {
        Self {
            objects,
            object_table: BTreeMap::new(),
        }
    }

    fn parse() {

    }
}