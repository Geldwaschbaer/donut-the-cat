mod dialog;
mod draw;
mod entity;
mod event;
mod map;
mod scene;

use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    draw::draw_p,
    entity::player::Player,
    map::Map,
    scene::{Scene, SceneManager, map_scene::MapScene},
};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Donut the Cat"),
        high_dpi: true,
        sample_count: 2,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time should go forward");
    rand::srand(since_the_epoch.as_millis() as u64);
    set_default_filter_mode(FilterMode::Nearest);
    let mut player = Player::new().await;
    let mut manager = SceneManager::new(MapScene::new(Map::new().await));
    manager.trigger_first_map_node(&mut player);

    debug!(
        "Welcome to Donut the cat!\nStarting with:\n - width: {}\n - height: {}",
        screen_width(),
        screen_height()
    );

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
            break;
        }

        if screen_width() >= 768.0 && screen_height() >= 556.0 {
            manager.draw(&player);
            manager.update(&mut player);
        } else {
            clear_background(BLACK);
            draw_p(
                &mut Vec2::new(30., 50.),
                "Your screen size does not meet\nthe minimum required resolution.\nPlease increase your screen size\nto at least 768x556 pixels!",
            );
        }

        next_frame().await
    }
}
