use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Mob {
    name: String,
    health: Health,
}

impl Mob {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_health(&self) -> &Health {
        &self.health
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
