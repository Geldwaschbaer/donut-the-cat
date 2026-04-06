use serde::Deserialize;

use crate::entity::Entity;

#[derive(Copy, Clone, Deserialize)]
pub struct Buff {
    r#type: BuffType,
    #[serde(default = "Default::default")]
    data: i32,
}

impl Buff {
    pub fn translate_damage_received(&self, value: i32) -> i32 {
        match self.get_type() {
            BuffType::Vulnerable => value * 3 / 2,
            BuffType::Block => (value - self.data).max(0),
            _ => value,
        }
    }

    pub fn translate_heal_received(&self, value: i32) -> i32 {
        match self.get_type() {
            BuffType::Burn => 0,
            _ => value,
        }
    }

    pub fn translate_damage_applied(&self, value: i32) -> i32 {
        match self.get_type() {
            BuffType::Weak => value / 2,
            BuffType::Berserk => value * 3 / 2,
            _ => value,
        }
    }

    pub fn end_of_turn(&self, source: &mut Entity) {
        match self.get_type() {
            BuffType::Poison => {
                source.hit_points -= self.data;
            }
            BuffType::Burn => {
                source.hit_points -= self.data;
            }
            _ => {}
        }
    }

    pub fn get_type(&self) -> &BuffType {
        &self.r#type
    }
}

#[derive(Copy, Clone, Deserialize)]
pub enum BuffType {
    Berserk,
    Block,
    Burn,
    Poison,
    Vulnerable,
    Weak,
}

impl BuffType {
    pub fn ordinal(&self) -> usize {
        *self as usize
    }
}
