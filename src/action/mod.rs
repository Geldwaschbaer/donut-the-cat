pub mod encounter;
mod return_map;

use crate::{player::Player, scene::SceneTransition};

pub trait Action {
    fn activate(&self, player: &mut Player) -> SceneTransition;
}
