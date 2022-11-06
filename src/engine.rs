use super::MoveKeys;

// Graphics
pub fn clear_background() {
    macroquad::prelude::clear_background(macroquad::prelude::GRAY);
}

pub fn draw_circle(x: f32, y: f32, r: f32) {
    macroquad::prelude::draw_circle_lines(x, y_transform(y), r, 2.0, macroquad::prelude::BLUE);
}

pub fn get_screen_height() -> f32 {
    macroquad::prelude::screen_height()
}

pub fn get_screen_width() -> f32 {
    macroquad::prelude::screen_width()
}

fn y_transform(y: f32) -> f32 {
    (macroquad::prelude::screen_height() as f32) - y
}

// Time
pub async fn await_next_frame() {
    macroquad::prelude::next_frame().await
}

// Input

pub fn get_active_move_keys() -> MoveKeys {
    MoveKeys {
        up: macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::W),
        down: macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::S),
        left: macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::A),
        right: macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::D),
    }
}
