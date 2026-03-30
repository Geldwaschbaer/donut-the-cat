#![allow(dead_code)]
#![allow(unused_variables)]

mod action;
mod map;
mod mob;
mod player;

use macroquad::prelude::*;

use crate::{map::map::Map, player::Player};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad Template"),
        high_dpi: true,
        sample_count: 2,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let map = Map::new();
    let player = Player::new(map.get_rooms().get(0).expect("expect any room exists"));

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        map.draw();

        next_frame().await
    }
}
