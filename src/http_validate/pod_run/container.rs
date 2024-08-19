use validify::Validify;
use serde::Deserialize;


#[derive(Deserialize, Validify)]
pub struct Container {
    pub image: String,
}
