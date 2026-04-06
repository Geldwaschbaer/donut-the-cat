use macroquad::{
    input::is_key_pressed,
    texture::{Texture2D, load_texture},
};

use crate::{
    entity::{Entity, buff::BuffType},
    event::Event,
    scene::KEY_CODES,
    scene::SceneTransition,
};

pub struct Player {
    map_position: usize,
    dialog_position: usize,
    entity: Entity,
    combat: Texture2D,
    icons: [Texture2D; 6],
}

impl Player {
    pub async fn new() -> Player {
        let texture =
            Texture2D::from_file_with_format(include_bytes!("../../assets/entity/donut.png"), None);
        let combat = Texture2D::from_file_with_format(
            include_bytes!("../../assets/backgrounds/battle-bg.png"),
            None,
        );
        Player {
            map_position: 0,
            dialog_position: 0,
            entity: Entity::new("Donut".into(), texture),
            combat,
            icons: [
                load_texture("assets/buff/berserk.png").await.unwrap(),
                load_texture("assets/buff/block.png").await.unwrap(),
                load_texture("assets/buff/burn.png").await.unwrap(),
                load_texture("assets/buff/poison.png").await.unwrap(),
                load_texture("assets/buff/vulnerability.png").await.unwrap(),
                load_texture("assets/buff/weakness.png").await.unwrap(),
            ],
        }
    }

    pub fn resolve_all(&mut self, events: &Vec<Event>) -> SceneTransition {
        let mut transition = SceneTransition::None;
        for event in events {
            transition = event.trigger(self);
        }
        return transition;
    }

    pub fn get_attack_used(&self) -> Option<usize> {
        for (index, _) in self.get_entity().get_attacks().iter().enumerate() {
            if is_key_pressed(KEY_CODES[index]) {
                return Some(index);
            }
        }
        None
    }

    pub fn set_map_position(&mut self, map_node: usize) {
        self.map_position = map_node;
    }

    pub fn get_map_position(&self) -> usize {
        self.map_position
    }

    pub fn get_dialog_position(&self) -> usize {
        self.dialog_position
    }

    pub fn set_dialog_position(&mut self, position: usize) {
        self.dialog_position = position;
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    pub fn get_combat_bg(&self) -> &Texture2D {
        &self.combat
    }

    pub fn get_buff_icon(&self, buff: &BuffType) -> &Texture2D {
        &self.icons[buff.ordinal()]
    }
}
