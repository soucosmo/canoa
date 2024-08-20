use lazy_static::lazy_static;
use validify::Validify;
use serde::Deserialize;
use regex::Regex;


lazy_static! {
    static ref MEMORY: Regex = Regex::new(r"^\d+([kKmMgG])?$").unwrap();
}

lazy_static! {
    static ref CPU: Regex = Regex::new(r"^\d+(\.\d{1,3})?$").unwrap();
}

#[derive(Clone, Validify, Deserialize, Debug)]
pub struct ResourcesValidate {
    #[modify(trim, lowercase)]
    #[validate(regex(MEMORY))]
    pub memory: String,
    #[modify(trim, lowercase)]
    #[validate(regex(CPU))]
    pub cpu: String,
}