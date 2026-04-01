use super::{Scene, SceneTransition};
use crate::{draw::*, mob::Mob, player::Player};
use macroquad::prelude::*;

pub struct CombatScene(Mob);

impl CombatScene {
    pub fn new(mob: Mob) -> CombatScene {
        CombatScene(mob)
    }

    pub fn get_mob(&self) -> &Mob {
        &self.0
    }
}

impl Scene for CombatScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);

        draw_lifebar(
            &mut Vec2::splat(0.),
            self.get_mob().get_name(),
            self.get_mob().get_health(),
        );
        draw_lifebar(
            &mut Vec2::new(screen_width() * 0.6, screen_height() * 0.4),
            "Player",
            player.get_health(),
        );

        draw_shadowbox(Rect::new(
            screen_width() * 0.05,
            screen_height() * 0.65,
            screen_width() * 0.9,
            screen_height() * 0.3,
        ));

        draw_text(
            &format!(
                "You encountered a wild {}! What do you do?",
                self.get_mob().get_name()
            ),
            screen_width() * 0.05 + 6.0,
            screen_height() * 0.65 + 22.0,
            22.0,
            BLACK,
        );
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        if self.get_mob().get_health().get_cur_health() > 0 {
            // combat continues

            SceneTransition::None
        } else {
            // killed the enemy
            SceneTransition::Pop
        }
    }
}
