use serde::{Serialize, Deserialize};
use super::Container;

#[derive(Clone, Serialize, Deserialize)]
pub struct Pod {
    pub name: String,
    pub containers: Vec<Container>,
}
