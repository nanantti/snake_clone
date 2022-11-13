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
    game_grid: grid::Grid<'a>,
    snake: player::Player<'a>,
    last_step_time: f64,
    generator: fruit_generator::FruitGenerator<'a>,
    fruit_location: (i32, i32),
}

impl Game<'_> {
    pub fn new(
        n_cells: &(i32, i32),
        step_duration_seconds_: f64,
        screen_size: (f32, f32),
        timestamp: f64,
        rng_seed: u64,
    ) -> Game {
        let snake_zero = Game::get_player_inital_location(*n_cells);
        let mut new_game = Game {
            step_duration_seconds: step_duration_seconds_,
            game_grid: grid::Grid {
                number_of_cells: n_cells,
                screen_size,
            },
            snake: player::Player::new(snake_zero, n_cells),
            last_step_time: timestamp,
            generator: fruit_generator::FruitGenerator::new(n_cells, rng_seed),
            fruit_location: (-1, -1),
        };
        new_game.fruit_location = new_game.roll_fruit_location();
        new_game
    }

    pub fn update(&mut self, current_time: f64, active_keys: &MoveKeys) {
        self.snake.draw(&self.game_grid);

        if current_time - self.last_step_time > self.step_duration_seconds {
            self.snake.update(active_keys, true);
            self.last_step_time = current_time;
        }
    }

    pub fn update_screen_size(&mut self, screen_size: (f32, f32)) {
        self.game_grid.update_screen_size(screen_size);
    }

    fn get_player_inital_location(screen_size: (i32, i32)) -> (i32, i32) {
        (screen_size.0 / 2, screen_size.1 / 2)
    }

    fn roll_fruit_location(&mut self) -> (i32, i32) {
        let mut location = self.generator.random_tile();
        while self.snake.collision(&location) {
            location = self.generator.random_tile();
        }
        location
    }
}

// TODO:
// add fruit
// add interaction between snake and fruit
// make game faster as snake grows
// add game over if snake touches self

#[macroquad::main("Snake")]
async fn main() {
    const STEP_DURATION_SECONDS: f64 = 1.0 / 10.0;
    let n_cells = (24, 24);
    let mut game = Game::new(
        &n_cells,
        STEP_DURATION_SECONDS,
        engine::get_screen_size(),
        engine::get_time(),
        0,
    );
    loop {
        engine::clear_background();
        game.update_screen_size(engine::get_screen_size());
        game.update(engine::get_time(), &engine::get_active_move_keys());
        engine::await_next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CELL_SIZE: (i32, i32) = (2, 2);
    const CELL_SIZE_3: (i32, i32) = (3, 3);
    const CELL_SIZE_4: (i32, i32) = (4, 4);

    #[test]
    fn snake_starts_at_middle_of_screen_even() {
        let game = Game::new(&CELL_SIZE_3, 1.0, (1.0, 1.0), 0.0, 0);
        assert_eq! { game.snake.get_head_location(), (1, 1) }
    }

    #[test]
    fn snake_starts_at_middle_of_screen_odd() {
        let game = Game::new(&CELL_SIZE_4, 1.0, (1.0, 1.0), 0.0, 0);
        assert_eq! { game.snake.get_head_location(), (2, 2) }
    }

    #[test]
    fn fruit_is_not_generated_in_space_occupied_by_snake() {
        for seed in 0..999 {
            let game = Game::new(&CELL_SIZE, 1.0, (1.0, 1.0), 0.0, seed);
            let snake_location = game.snake.get_head_location();

            assert_ne! {game.fruit_location, snake_location}
        }
    }
}
