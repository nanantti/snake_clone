mod engine;
mod fruit_generator;
mod grid;
mod player;

pub struct MoveKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

// TODO:
// add fruit
// add interaction between snake and fruit
// make game faster as snake grows
// add game over if snake touches self

fn roll_fruit_location(
    snake: &player::Player,
    gen: &mut fruit_generator::FruitGenerator,
) -> (i32, i32) {
    let mut location = gen.random_tile();
    while snake.collision(&location) {
        location = gen.random_tile();
    }
    location
}

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
        player.draw(&grid);

        let current_time = engine::get_time();
        if current_time - last_step_time > STEP_DURATION_SECONDS {
            player.update(&engine::get_active_move_keys(), true);
            last_step_time = current_time;
        }
        engine::await_next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CELL_SIZE: (i32, i32) = (2, 2);

    #[test]
    fn fruit_generated_where_snake_is_gets_rerolled() {
        let mut gen = fruit_generator::FruitGenerator::new(&CELL_SIZE);
        let player = player::Player::new((1, 0), &CELL_SIZE);
        gen.set_rand_seed(0);

        assert_eq! {gen.random_tile(), (1, 0)}

        assert_eq! {roll_fruit_location(&player, &mut gen), (1, 1)}
    }
}
