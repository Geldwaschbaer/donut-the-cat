pub mod enemy;
pub mod player;

use async_from::{AsyncFrom, async_trait};
use macroquad::texture::{FilterMode, Texture2D, load_texture};
use serde::Deserialize;

#[derive(Clone)]
pub struct Entity {
    name: String,
    health: Health,
    count: u32,
    attacks: Vec<Attack>,
    texture: Texture2D,
}

pub const fn default_count() -> u32 {
    1
}

impl Entity {
    pub fn new(name: String, health: Health, attacks: Vec<Attack>, texture: Texture2D) -> Entity {
        Entity {
            name,
            health,
            count: 1,
            attacks,
            texture,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_health(&self) -> &Health {
        &self.health
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn use_attack(&mut self, attack: usize, target: &mut Entity) {
        let attack = self.attacks.get(attack).expect("expected attack exists");
        target.health.cur_health -= attack.damage as i32
            * if attack.area_of_effect {
                target.count as i32
            } else {
                1
            };
        self.health.cur_health = (self.health.cur_health
            + attack.heal as i32
                * if attack.area_of_effect {
                    self.count as i32
                } else {
                    1
                })
        .min(self.health.max_health as i32);
    }

    pub fn get_attacks(&self) -> &Vec<Attack> {
        &self.attacks
    }

    pub fn get_attacks_mut(&mut self) -> &mut Vec<Attack> {
        &mut self.attacks
    }

    pub fn get_texture(&self) -> &Texture2D {
        &self.texture
    }
}

#[derive(Deserialize)]
pub struct EntityBuilder {
    name: String,
    health: Health,
    #[serde(default = "default_count")]
    count: u32,
    attacks: Vec<Attack>,
    texture: String,
}

#[async_trait]
impl AsyncFrom<EntityBuilder> for Entity {
    async fn async_from(value: EntityBuilder) -> Self {
        let texture = load_texture(&value.texture).await.expect("texture exists");
        texture.set_filter(FilterMode::Nearest);
        Entity {
            name: value.name,
            health: value.health,
            count: value.count,
            attacks: value.attacks,
            texture,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Health {
    cur_health: i32,
    max_health: u32,
}

impl Health {
    pub fn new(health: u32) -> Health {
        Health {
            cur_health: health as i32,
            max_health: health,
        }
    }

    pub fn get_cur_health(&self) -> i32 {
        self.cur_health
    }

    pub fn get_max_health(&self) -> u32 {
        self.max_health
    }
}

#[derive(Clone, Deserialize)]
pub struct Attack {
    description: String,
    damage: u32,
    heal: u32,
    area_of_effect: bool,
}

impl Attack {
    pub fn new(description: String, damage: u32, heal: u32, area_of_effect: bool) -> Attack {
        Attack {
            description,
            damage,
            heal,
            area_of_effect,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
