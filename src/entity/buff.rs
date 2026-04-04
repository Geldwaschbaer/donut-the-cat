use macroquad::logging::debug;
use serde::Deserialize;

use crate::entity::Entity;

#[derive(Clone, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Buff {
    Weak,
    Vulnerable,
    Berserk,
    Poison(i32),
    Burn(i32),
    Block(i32),
}

impl Buff {
    pub fn translate_damage_received(&self, value: i32) -> i32 {
        match self {
            Self::Vulnerable => value * 3 / 2,
            Self::Block(block) => (value - block).max(0),
            _ => value,
        }
    }

    pub fn translate_heal_received(&self, value: i32) -> i32 {
        match self {
            Self::Burn(_) => 0,
            _ => value,
        }
    }

    pub fn translate_damage_applied(&self, value: i32) -> i32 {
        match self {
            Self::Weak => value / 2,
            Self::Berserk => value * 3 / 2,
            _ => value,
        }
    }

    pub fn end_of_turn(&self, source: &mut Entity) {
        match self {
            Self::Poison(value) => {
                source.hit_points -= value;
            }
            Self::Burn(value) => {
                debug!("burn value: {}", value);
                source.hit_points -= value;
            }
            _ => {}
        }
    }
}
