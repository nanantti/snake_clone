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

pub struct Game<'a> {
    step_duration_seconds: f64,
    n_cells: &'a(i32, i32),
    game_grid: grid::Grid<'a>,
    snake: player::Player<'a>,
    last_step_time: f64,
}

impl Game<'_> {
    pub fn new(n_cells: &(i32, i32), step_duration_seconds_: f64) -> Game {
        Game {
            n_cells: n_cells,
            step_duration_seconds: step_duration_seconds_,
            game_grid: grid::Grid {
                number_of_cells: n_cells,
                screen_size: (engine::get_screen_width(), engine::get_screen_height()),
            },
            snake: player::Player::new((0, 0), n_cells),
            last_step_time: engine::get_time(),
        }
    }

    pub fn update(&mut self) {
        self.game_grid.update_screen_size((engine::get_screen_width(), engine::get_screen_height()));
        engine::clear_background();
        self.snake.draw(&self.game_grid);

        let current_time = engine::get_time();
        if current_time - self.last_step_time > self.step_duration_seconds {
            self.snake.update(&engine::get_active_move_keys(), true);
            self.last_step_time = current_time;
        }
    }
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
    let mut game = Game::new(&n_cells, STEP_DURATION_SECONDS);
    loop {
        game.update();
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
