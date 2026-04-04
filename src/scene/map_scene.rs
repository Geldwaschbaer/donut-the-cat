use crate::{
    draw::{draw_h1, draw_ol, draw_p, draw_shadowbox},
    entity::{Stat, player::Player},
    map::Map,
    scene::{Scene, SceneTransition},
};
use macroquad::prelude::*;

pub struct MapScene(Map);

impl MapScene {
    pub fn new(map: Map) -> MapScene {
        MapScene(map)
    }

    pub fn get_map(&self) -> &Map {
        &self.0
    }

    pub fn get_map_mut(&mut self) -> &mut Map {
        &mut self.0
    }
}

impl Scene for MapScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);
        let room = self
            .get_map()
            .get_rooms()
            .get(player.get_map_position())
            .expect("expect exists");
        for neig in room.get_neighbours() {
            let neig = self
                .get_map()
                .get_rooms()
                .get(*neig)
                .expect("element exists");
            draw_line(
                room.get_position().x * screen_width(),
                room.get_position().y * screen_height(),
                neig.get_position().x * screen_width(),
                neig.get_position().y * screen_height(),
                3.,
                DARKPURPLE,
            );
            draw_arc(
                neig.get_position().x * screen_width(),
                neig.get_position().y * screen_height(),
                120,
                26.,
                0.,
                3.,
                360.,
                DARKPURPLE,
            )
        }
        self.get_map().draw();

        draw_shadowbox(Rect::new(
            screen_width() * 0.8,
            screen_height() * 0.25,
            screen_width() * 0.18,
            screen_height() * 0.2,
        ));
        let mut pos = Vec2::new(screen_width() * 0.8 + 20., screen_height() * 0.25 + 40.0);
        draw_h1(&mut pos, "Legende");
        draw_ol(
            &mut pos,
            ["Endboss", "Boss", "Enemy", "Mystery"].into_iter(),
        );

        draw_shadowbox(Rect::new(
            screen_width() * 0.8,
            screen_height() * 0.55,
            screen_width() * 0.18,
            screen_height() * 0.2,
        ));
        let mut pos = Vec2::new(screen_width() * 0.8 + 20., screen_height() * 0.55 + 40.0);
        draw_h1(&mut pos, "Your Stats");
        draw_p(
            &mut pos,
            &format!(
                "Str: {}\nDex: {}\nCon: {}\nInt: {}",
                player.get_entity().get_stat(&Stat::Str),
                player.get_entity().get_stat(&Stat::Dex),
                player.get_entity().get_stat(&Stat::Con),
                player.get_entity().get_stat(&Stat::Int)
            ),
        );
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let room = self.get_map().get_room(player.get_map_position());
            for neig_num in room.get_neighbours() {
                let target = *neig_num;
                let neig = self.get_map().get_room(target);
                let dx = neig.get_position().x * screen_width() - x;
                let dy = neig.get_position().y * screen_height() - y;
                if (dx * dx + dy * dy).sqrt() < 26.0 {
                    player.set_map_position(target);
                    self.0
                        .get_rooms_mut()
                        .get_mut(target)
                        .expect("expected room to enter exists")
                        .mark_visited();
                    return self.get_map().get_room(target).get_event().trigger(player);
                }
            }
        }
        SceneTransition::None
    }
}
