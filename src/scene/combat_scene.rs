use crate::{
    draw::*,
    entity::{enemy::Enemy, player::Player},
    scene::{Scene, SceneTransition, game_over_scene::GameOverScene},
};
use macroquad::prelude::*;

pub struct CombatScene {
    enemy: Enemy,
    cooldown: f32,
}

impl CombatScene {
    pub fn new(enemy: Enemy) -> CombatScene {
        CombatScene {
            enemy,
            cooldown: 0.0,
        }
    }

    pub fn get_enemy(&self) -> &Enemy {
        &self.enemy
    }
}

impl Scene for CombatScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);
        draw_texture_ex(
            player.get_combat_bg(),
            0.02 * screen_width(),
            0.02 * screen_height(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width() * 0.96, screen_height() * 0.6)),
                ..Default::default()
            },
        );

        draw_lifebar(&mut Vec2::splat(0.), self.get_enemy().get_entity());
        if (self.cooldown * 10.0) as i32 & 1 == 0 {
            draw_texture_ex(
                self.get_enemy().get_entity().get_texture(),
                screen_width() * 0.6,
                screen_height() * 0.1,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(256.0)),
                    ..Default::default()
                },
            );
        }
        draw_lifebar(
            &mut Vec2::new(screen_width() * 0.6, screen_height() * 0.4),
            player.get_entity(),
        );
        if (self.cooldown * 10.0) as i32 & 1 == 0 {
            draw_texture_ex(
                player.get_entity().get_texture(),
                screen_width() * 0.1,
                screen_height() * 0.25,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(256.0)),
                    ..Default::default()
                },
            );
        }

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
        draw_attacks(&mut pos, player, player.get_entity().get_attacks().iter());
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        self.cooldown = (self.cooldown - get_frame_time()).max(0.0);
        if self.cooldown == 0.0 {
            if !player.get_entity().is_alive() {
                return SceneTransition::Push(Box::new(GameOverScene::new(format!(
                    "You were killed by: {}",
                    self.get_enemy().get_entity().get_name()
                ))));
            } else if self.get_enemy().get_entity().is_alive() {
                // combat continues
                let attack_used = player.get_attack_used();
                if let Some(attack) = attack_used {
                    self.enemy.get_entity_mut().end_turn();
                    player
                        .get_entity_mut()
                        .use_attack(attack, self.enemy.get_entity_mut());
                    player.get_entity_mut().end_turn();
                    if self.get_enemy().get_entity().is_alive() {
                        let attack_count = self.get_enemy().get_entity().get_attacks().len();
                        if attack_count > 0 {
                            let attack = rand::gen_range(0, attack_count);
                            self.enemy
                                .get_entity_mut()
                                .use_attack(attack, player.get_entity_mut());
                        }
                    }
                    self.cooldown = 0.5;
                }
                SceneTransition::None
            } else {
                if !self.get_enemy().get_entity().is_alive() {
                    return player.resolve_all(self.get_enemy().get_on_death());
                }
                // killed the enemy
                SceneTransition::Clear
            }
        } else {
            SceneTransition::None
        }
    }
}
