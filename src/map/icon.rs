use serde::Deserialize;

#[derive(Deserialize)]
pub enum MapIcon {
    Boss,
    Endboss,
    Enemy,
    Mystery,
    Shop,
    Start,
}
