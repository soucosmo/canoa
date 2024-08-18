use serde::{Serialize, Deserialize};
use super::Pod;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub pods: Vec<Pod>,
}
