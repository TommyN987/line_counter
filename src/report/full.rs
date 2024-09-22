use serde::Serialize;

use super::{analytics::Analytics, tree::Tree};

#[derive(Debug, Serialize)]
pub struct Full {
    pub analytics: Analytics,
    pub tree: Tree,
}

impl Full {
    pub fn new(analytics: Analytics, tree: Tree) -> Self {
        Self { analytics, tree }
    }
}
