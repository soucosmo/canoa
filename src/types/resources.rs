use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Resources {
    pub cpu: String,
    pub memory: String,
}
