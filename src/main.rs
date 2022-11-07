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
// plot snake body
// add fruit
// add interaction between snake and fruit
// make game faster as snake grows

#[macroquad::main("Snake")]
async fn main() {
    const frame_duration_seconds: f64 = 1.0 / 10.0;
    let n_cells = (24, 24);
    let mut grid = grid::Grid {
        number_of_cells: &n_cells,
        screen_size: (engine::get_screen_width(), engine::get_screen_height()),
    };
    let mut player = player::Player::new((12, 12), &n_cells);
    loop {
        let timestamp_start_frame = engine::get_time();
        grid.update_screen_size((engine::get_screen_width(), engine::get_screen_height()));
        engine::clear_background();
        player.update(&engine::get_active_move_keys());
        let player_coord = grid.get_cell_center(player.get_location());
        engine::draw_circle(player_coord.0, player_coord.1, grid.get_cell_size() * 0.50);

        while engine::get_time() - timestamp_start_frame < frame_duration_seconds {}
        engine::await_next_frame().await
    }
}
