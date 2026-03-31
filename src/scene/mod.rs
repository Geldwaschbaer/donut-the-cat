pub mod dialog_scene;
pub mod map_scene;

use crate::player::Player;

pub trait Scene {
    fn draw(&self, player: &Player);
    fn update(&mut self, player: &mut Player) -> SceneTransition;
}

pub enum SceneTransition {
    // No transition is happening
    None,
    // Switch to the specified scene
    Switch(Box<dyn Scene>),
    // Return to the global map scene
    Return,
}
