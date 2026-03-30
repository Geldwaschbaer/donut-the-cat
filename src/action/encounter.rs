use crate::action::Action;

use crate::mob::Mob;
use crate::player::Player;

#[derive(Default)]
pub struct Encounter {
    mobs: Vec<Mob>,
}

impl Action for Encounter {
    fn activate(&self, player: &mut Player) {
        todo!()
    }
}
