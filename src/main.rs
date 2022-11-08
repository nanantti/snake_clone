mod engine;
mod grid;
mod player;

pub struct MoveKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

fn draw_player(pl: &player::Player, gd: &grid::Grid) {
    let player_coord = gd.get_cell_center(pl.get_head_location());
    engine::draw_circle(player_coord.0, player_coord.1, gd.get_cell_size() * 0.50);
}

// TODO:
// plot snake body
// add fruit
// add interaction between snake and fruit
// make game faster as snake grows

#[macroquad::main("Snake")]
async fn main() {
    const STEP_DURATION_SECONDS: f64 = 1.0 / 10.0;
    let n_cells = (24, 24);
    let mut grid = grid::Grid {
        number_of_cells: &n_cells,
        screen_size: (engine::get_screen_width(), engine::get_screen_height()),
    };
    let mut player = player::Player::new((12, 12), &n_cells);
    let mut last_step_time = engine::get_time();
    loop {
        grid.update_screen_size((engine::get_screen_width(), engine::get_screen_height()));
        engine::clear_background();
        draw_player(&player, &grid);

        let current_time = engine::get_time();
        if current_time - last_step_time > STEP_DURATION_SECONDS {
            player.update(&engine::get_active_move_keys());
            last_step_time = current_time;
        }
        engine::await_next_frame().await
    }
}
