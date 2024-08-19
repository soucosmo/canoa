use std::collections::{BTreeMap, BTreeSet};
use serde::{Serialize, Deserialize};
use super::Pod;
use uuid::Uuid;


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub namespaces: BTreeSet<String>,
    pub pods: BTreeMap<Uuid, Pod>,
}
