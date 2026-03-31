use crate::{
    dialog::Dialog,
    player::Player,
    scene::{Scene, SceneTransition},
};
use macroquad::prelude::*;

pub struct DialogScene(Dialog);

impl DialogScene {
    pub fn new(dialog: Dialog) -> DialogScene {
        DialogScene(dialog)
    }

    pub fn get_dialog(&self) -> &Dialog {
        &self.0
    }
}

impl Scene for DialogScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);
        draw_rectangle(
            screen_width() * 0.1,
            screen_height() * 0.5,
            screen_width() * 0.8,
            screen_height() * 0.4,
            DARKGRAY,
        );
        draw_text(
            self.get_dialog().get_title(),
            screen_width() * 0.15,
            screen_height() * 0.55,
            30.0,
            WHITE,
        );
        let dialog_box = self
            .get_dialog()
            .get_dialogs()
            .get(player.get_dialog_position())
            .expect("expect dialog node exists");
        for (index, dialog_option) in dialog_box.get_options().iter().enumerate() {
            let y = screen_height() * 0.55 + 35. + index as f32 * 25.0;
            draw_text(
                &format!("{}. ", index + 1),
                screen_width() * 0.16,
                y,
                22.0,
                WHITE,
            );
            draw_text(
                dialog_option.get_description(),
                screen_width() * 0.20,
                y,
                22.,
                RED,
            );
        }
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        let dialog_box = self
            .get_dialog()
            .get_dialogs()
            .get(player.get_dialog_position())
            .expect("expect dialog node exists");
        for (index, dialog_option) in dialog_box.get_options().iter().enumerate() {}
        SceneTransition::None
    }
}
