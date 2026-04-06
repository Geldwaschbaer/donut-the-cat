use serde::Deserialize;

use crate::entity::Entity;

#[derive(Copy, Clone, Debug, Deserialize)]
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

    pub fn display(&self) -> String {
        if self.data > 0 {
            format!("{} ({})", self.get_type().name(), self.data)
        } else {
            self.get_type().name().into()
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum BuffType {
    Berserk,
    Block,
    Burn,
    Poison,
    Vulnerable,
    Weak,
}

impl BuffType {
    pub fn name(&self) -> &str {
        match self {
            Self::Berserk => "Berserk",
            Self::Block => "Block",
            Self::Burn => "Burn",
            Self::Poison => "Poison",
            Self::Vulnerable => "Vulnerable",
            Self::Weak => "Weak",
        }
    }

    pub fn ordinal(&self) -> usize {
        *self as usize
    }
}
