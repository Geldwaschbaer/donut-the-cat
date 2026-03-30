pub mod encounter;

use crate::player::Player;

pub trait Action {
    fn activate(&self, player: &mut Player);
}
