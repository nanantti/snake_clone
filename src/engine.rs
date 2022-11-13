use super::MoveKeys;

pub const SNAKE_COLOR: macroquad::prelude::Color = macroquad::prelude::RED;
const BACKGROUND_COLOR: macroquad::prelude::Color = macroquad::prelude::BLACK;
pub const FRUIT_COLOR: macroquad::prelude::Color = macroquad::prelude::GREEN;

// Graphics
pub fn clear_background() {
    macroquad::prelude::clear_background(BACKGROUND_COLOR);
}

pub fn draw_circle(x: f32, y: f32, r: f32, color: macroquad::prelude::Color) {
    macroquad::prelude::draw_circle(x, y, r, color);
}

pub fn get_screen_size() -> (f32, f32) {
    (get_screen_width(), get_screen_height())
}

fn get_screen_height() -> f32 {
    macroquad::prelude::screen_height()
}

fn get_screen_width() -> f32 {
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
        SNAKE_COLOR,
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

// Random
pub fn set_rand_seed(seed: u64) {
    macroquad::rand::srand(seed);
}

pub fn gen_range<T: macroquad::rand::RandomRange>(low: T, high: T) -> T {
    macroquad::rand::gen_range::<T>(low, high)
}
