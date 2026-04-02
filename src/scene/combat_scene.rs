use crate::{
    draw::*,
    entity::{enemy::Enemy, player::Player},
    scene::{Scene, SceneTransition},
};
use macroquad::prelude::*;

pub struct CombatScene(Enemy);

impl CombatScene {
    pub fn new(mob: Enemy) -> CombatScene {
        CombatScene(mob)
    }

    pub fn get_enemy(&self) -> &Enemy {
        &self.0
    }
}

impl Scene for CombatScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);

        draw_lifebar(&mut Vec2::splat(0.), self.get_enemy().get_entity());
        draw_lifebar(
            &mut Vec2::new(screen_width() * 0.6, screen_height() * 0.4),
            player.get_entity(),
        );

        draw_shadowbox(Rect::new(
            screen_width() * 0.05,
            screen_height() * 0.65,
            screen_width() * 0.9,
            screen_height() * 0.3,
        ));

        let mut pos = Vec2::new(screen_width() * 0.15, screen_height() * 0.75);
        draw_h1(&mut pos, self.get_enemy().get_entity().get_name());
        draw_p(
            &mut pos,
            &format!(
                "You encountered a wild {}! What do you do?",
                self.get_enemy().get_entity().get_name()
            ),
        );
        draw_ol(&mut pos, ["A", "B", "C"].into_iter());
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        if self.get_enemy().get_entity().get_health().get_cur_health() > 0 {
            // combat continues

            SceneTransition::None
        } else {
            // killed the enemy
            SceneTransition::Pop
        }
    }
}
