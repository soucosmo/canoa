use validify::Validify;
use serde::Deserialize;
mod container;
mod resources;

pub use resources::ResourcesValidate;

#[derive(Deserialize, Validify, Debug)]
pub struct PodRunValidate {
    #[modify(trim, lowercase)]
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[modify(trim, lowercase)]
    #[validate(length(min = 3, max = 20))]
    pub namespace: String,
    #[modify(trim)]
    #[validate(length(min = 3), contains(value = ":", message = "It is necessary to inform the image with the tag ex: ubuntu:20.04"))]
    pub image: String,
    pub replicas: u16,
    pub resources: Option<ResourcesValidate>,
}
