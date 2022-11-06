mod engine;
mod grid;

#[macroquad::main("Snake")]
async fn main() {
    loop {
        engine::clear_background();
        engine::await_next_frame().await
    }
}
