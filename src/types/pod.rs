use serde::{Serialize, Deserialize};
use super::{Container, Resources};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pod {
    pub name: String,
    pub namespace: String,
    pub image: String,
    pub replicas: u16,
    pub containers: Vec<Container>,
    pub resources: Resources,
}
