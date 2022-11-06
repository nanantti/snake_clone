mod engine;
mod grid;
mod player;

pub struct MoveKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[macroquad::main("Snake")]
async fn main() {
    let mut grid = grid::Grid {
        number_of_cells: (9, 9),
        screen_size: (engine::get_screen_width(), engine::get_screen_height()),
    };
    let player_index = (1, 1);
    loop {
        grid.update_screen_size((engine::get_screen_width(), engine::get_screen_height()));
        engine::clear_background();
        let player_coord = grid.get_cell_center(player_index);
        engine::draw_circle(player_coord.0, player_coord.1, grid.get_cell_size() * 0.50);
        engine::await_next_frame().await
    }
}
