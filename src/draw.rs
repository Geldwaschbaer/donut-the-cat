use crate::mob::Health;
use macroquad::prelude::*;

pub const ACTIVATED: Color = Color::from_hex(0x1b252e);
pub const AVAILABLE: Color = Color::from_hex(0x585858);

pub fn draw_lifebar(offset: Vec2, name: &str, health: &Health) {
    draw_shadowbox(Rect::new(
        screen_width() * 0.05 + offset.x,
        screen_height() * 0.05 + offset.y,
        screen_width() * 0.3,
        screen_height() * 0.1,
    ));

    draw_text(
        name,
        screen_width() * 0.05 + offset.x + 5.0,
        screen_height() * 0.05 + offset.y + 5.0 + 12.0,
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

    draw_shadowbox_ex(
        Rect::new(
            screen_width() * 0.15 + offset.x,
            screen_height() * 0.08 + offset.y,
            screen_width() * 0.15,
            25.,
        ),
        DrawShadowboxParams {
            padding: Rect::new(2.0, 2.0, 2.0, 3.0),
            fill: PINK,
            stroke: BLACK,
        },
    );
}

pub fn draw_shadowbox(rect: Rect) {
    draw_shadowbox_ex(rect, DrawShadowboxParams::default());
}

pub struct DrawShadowboxParams {
    pub padding: Rect,
    pub fill: Color,
    pub stroke: Color,
}

impl Default for DrawShadowboxParams {
    fn default() -> DrawShadowboxParams {
        DrawShadowboxParams {
            padding: Rect::new(2.0, 2.0, 2.0, 10.0),
            fill: WHITE,
            stroke: BLACK,
        }
    }
}

pub fn draw_shadowbox_ex(rect: Rect, params: DrawShadowboxParams) {
    let padding = params.padding;
    draw_rectangle(
        rect.x - padding.x,
        rect.y - padding.y,
        rect.w + padding.x + padding.w,
        rect.h + padding.y + padding.h,
        params.stroke,
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, params.fill);
}
