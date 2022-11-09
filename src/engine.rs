use super::MoveKeys;

// Graphics
pub fn clear_background() {
    macroquad::prelude::clear_background(macroquad::prelude::GRAY);
}

pub fn draw_circle(x: f32, y: f32, r: f32) {
    macroquad::prelude::draw_circle(x, y, r, macroquad::prelude::BLUE);
}

pub fn get_screen_height() -> f32 {
    macroquad::prelude::screen_height()
}

pub fn get_screen_width() -> f32 {
    macroquad::prelude::screen_width()
}

pub fn draw_triangle(x: f32, y: f32, base: f32, rotation_degrees: f32) {
    let center = macroquad::math::Vec2 { x, y };
    let a_half = 0.5 * base;
    let rotation =
        macroquad::math::Vec2::from_angle(std::f32::consts::PI * (rotation_degrees) / 180.0);
    let a1 = macroquad::math::Vec2 { x: 0.0, y: -a_half };
    let a2 = macroquad::math::Vec2 {
        x: -a_half,
        y: a_half,
    };
    let a3 = macroquad::math::Vec2 {
        x: a_half,
        y: a_half,
    };
    macroquad::shapes::draw_triangle(
        center + rotation.rotate(a1),
        center + rotation.rotate(a2),
        center + rotation.rotate(a3),
        macroquad::prelude::BLUE,
    );
}

// Time
pub async fn await_next_frame() {
    macroquad::prelude::next_frame().await
}

pub fn get_time() -> f64 {
    macroquad::prelude::get_time()
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
