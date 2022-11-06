mod engine;
mod grid;
mod player;

pub struct MoveKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

// TODO:
// limit FPS
// add wraparound
// plot snake body
// add fruit
// add interaction between snake and fruit
// make game faster as snake grows

#[macroquad::main("Snake")]
async fn main() {
    let mut grid = grid::Grid {
        number_of_cells: (99, 99),
        screen_size: (engine::get_screen_width(), engine::get_screen_height()),
    };
    let mut player = player::Player::new((50, 50));
    loop {
        grid.update_screen_size((engine::get_screen_width(), engine::get_screen_height()));
        engine::clear_background();
        player.update(&engine::get_active_move_keys());
        let player_coord = grid.get_cell_center(player.get_location());
        engine::draw_circle(player_coord.0, player_coord.1, grid.get_cell_size() * 0.50);
        engine::await_next_frame().await
    }
}
