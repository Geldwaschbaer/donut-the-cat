use super::{Scene, SceneTransition};
use crate::{
    mob::{Health, Mob},
    player::Player,
};
use macroquad::prelude::*;

pub struct CombatScene(Mob);

impl CombatScene {
    pub fn new(mob: Mob) -> CombatScene {
        CombatScene(mob)
    }

    pub fn draw_lifebar(&self, offset: Vec2, name: &str, health: &Health) {
        draw_rectangle(
            screen_width() * 0.05 + offset.x,
            screen_height() * 0.05 + offset.y,
            screen_width() * 0.3,
            screen_height() * 0.1,
            BLACK,
        );
        draw_rectangle(
            screen_width() * 0.05 + offset.x + 2.0,
            screen_height() * 0.05 + offset.y + 2.0,
            screen_width() * 0.3 - 4.0,
            screen_height() * 0.1 - 4.0,
            WHITE,
        );
        draw_text(
            name,
            screen_width() * 0.05 + offset.x + 5.0,
            screen_height() * 0.05 + offset.y + 5.0 + 22.0,
            22.0,
            BLACK,
        );

        draw_text(
            &format!(
                "hp: {}/{}",
                health.get_cur_health(),
                health.get_max_health()
            ),
            screen_width() * 0.05 + offset.x + 5.0,
            screen_height() * 0.05 + offset.y + 5.0 + 36.0,
            14.0,
            BLACK,
        );
    }

    pub fn get_mob(&self) -> &Mob {
        &self.0
    }
}

impl Scene for CombatScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);

        draw_rectangle(
            screen_width() * 0.05,
            screen_height() * 0.05,
            screen_width() * 0.3,
            screen_height() * 0.1,
            GRAY,
        );
        self.draw_lifebar(
            Vec2::ZERO,
            self.get_mob().get_name(),
            self.get_mob().get_health(),
        );
        self.draw_lifebar(
            Vec2::new(screen_width() * 0.5, screen_height() * 0.4),
            "Player",
            player.get_health(),
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
