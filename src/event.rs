use crate::{
    dialog::Dialog,
    entity::{enemy::Enemy, player::Player},
    scene::{SceneTransition, combat_scene::CombatScene, dialog_scene::DialogScene},
};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    // nothing happens when this event is triggered
    Nothing,
    // return back to the global map
    ReturnToMap,
    // opens a new dialog
    OpenDialog(Dialog),

    EnterCombat(Enemy),
}

impl Event {
    pub fn trigger(&self, player: &mut Player) -> SceneTransition {
        match self {
            Event::Nothing => SceneTransition::None,
            Event::ReturnToMap => SceneTransition::Pop,
            Event::OpenDialog(dialog) => {
                SceneTransition::Push(Box::new(DialogScene::new(dialog.clone())))
            }
            Event::EnterCombat(mob) => {
                SceneTransition::Push(Box::new(CombatScene::new(mob.clone())))
            }
        }
    }
}
