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
}

impl Game<'_> {
    pub fn new(
        n_cells: &(i32, i32),
        step_duration_seconds_: f64,
        screen_size: (f32, f32),
        timestamp: f64,
    ) -> Game {
        let snake_zero = Game::get_player_inital_location(*n_cells);
        Game {
            step_duration_seconds: step_duration_seconds_,
            game_grid: grid::Grid {
                number_of_cells: n_cells,
                screen_size: screen_size,
            },
            snake: player::Player::new(snake_zero, n_cells),
            last_step_time: timestamp,
        }
    }

    pub fn update(&mut self) {
        engine::clear_background();
        self.snake.draw(&self.game_grid);

        let current_time = engine::get_time();
        if current_time - self.last_step_time > self.step_duration_seconds {
            self.snake.update(&engine::get_active_move_keys(), true);
            self.last_step_time = current_time;
        }
    }

    pub fn update_screen_size(&mut self, screen_size: (f32, f32)) {
        self.game_grid.update_screen_size(screen_size);
    }

    fn get_player_inital_location(screen_size: (i32, i32)) -> (i32, i32) {
        (screen_size.0 / 2, screen_size.1 / 2)
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

fn get_screen_size() -> (f32, f32) {
    (engine::get_screen_width(), engine::get_screen_height())
}

#[macroquad::main("Snake")]
async fn main() {
    const STEP_DURATION_SECONDS: f64 = 1.0 / 10.0;
    let n_cells = (24, 24);
    let mut game = Game::new(
        &n_cells,
        STEP_DURATION_SECONDS,
        get_screen_size(),
        engine::get_time(),
    );
    loop {
        game.update_screen_size(get_screen_size());
        game.update();
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
        let game = Game::new(&CELL_SIZE_3, 1.0, (1.0, 1.0), 0.0);
        assert_eq! { game.snake.get_head_location(), (1, 1) }
    }

    #[test]
    fn snake_starts_at_middle_of_screen_odd() {
        let game = Game::new(&CELL_SIZE_4, 1.0, (1.0, 1.0), 0.0);
        assert_eq! { game.snake.get_head_location(), (2, 2) }
    }

    #[test]
    fn fruit_generated_where_snake_is_gets_rerolled() {
        let mut gen = fruit_generator::FruitGenerator::new(&CELL_SIZE);
        let player = player::Player::new((1, 0), &CELL_SIZE);
        gen.set_rand_seed(0);

        assert_eq! {gen.random_tile(), (1, 0)}

        assert_eq! {roll_fruit_location(&player, &mut gen), (1, 1)}
    }
}
