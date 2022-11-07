use super::MoveKeys;

pub struct Player<'a> {
    pub location: (i32, i32),
    direction: MoveDirection,
    number_of_cells: &'a (i32, i32),
}

impl Player<'_> {
    pub fn new(initial_location: (i32, i32), n_cells: &(i32, i32)) -> Player {
        Player {
            location: initial_location,
            direction: MoveDirection::Left,
            number_of_cells: n_cells,
        }
    }

    pub fn get_location(&self) -> (i32, i32) {
        self.location
    }

    pub fn update(&mut self, keys: &MoveKeys) {
        self.update_direction(&keys);
        self.update_location();
        self.check_warparound();
    }

    fn update_direction(&mut self, keys: &MoveKeys) {
        if self.is_reverse_direction(keys) {
            return;
        }
        match keys {
            MoveKeys { up: true, .. } => self.direction = MoveDirection::Up,
            MoveKeys { down: true, .. } => self.direction = MoveDirection::Down,
            MoveKeys { left: true, .. } => self.direction = MoveDirection::Left,
            MoveKeys { right: true, .. } => self.direction = MoveDirection::Right,
            MoveKeys {
                up: false,
                down: false,
                left: false,
                right: false,
            } => {}
        }
    }

    fn is_reverse_direction(&self, keys: &MoveKeys) -> bool {
        match self.direction {
            MoveDirection::Up => keys.down,
            MoveDirection::Down => keys.up,
            MoveDirection::Left => keys.right,
            MoveDirection::Right => keys.left,
        }
    }

    fn update_location(&mut self) {
        match self.direction {
            MoveDirection::Up => {
                self.location.1 -= 1;
            }
            MoveDirection::Down => {
                self.location.1 += 1;
            }
            MoveDirection::Left => {
                self.location.0 -= 1;
            }
            MoveDirection::Right => {
                self.location.0 += 1;
            }
        }
    }

    fn check_warparound(&mut self) {
        if self.location.0 < 0 {
            self.location.0 = self.number_of_cells.0;
        }
        if self.location.0 > self.number_of_cells.0 {
            self.location.0 = 0;
        }
        if self.location.1 < 0 {
            self.location.1 = self.number_of_cells.1;
        }
        if self.location.1 > self.number_of_cells.1 {
            self.location.1 = 0;
        }
    }
}

enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests_grid {
    use super::*;
    const no_press: MoveKeys = MoveKeys {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    const up_press: MoveKeys = MoveKeys {
        up: true,
        down: false,
        left: false,
        right: false,
    };

    const down_press: MoveKeys = MoveKeys {
        up: false,
        down: true,
        left: false,
        right: false,
    };

    const right_press: MoveKeys = MoveKeys {
        up: false,
        down: false,
        left: false,
        right: true,
    };

    const cell_size: (i32, i32) = (5, 5);

    #[test]
    fn player_starts_moving_left() {
        let mut player = Player::new((5, 5), &cell_size);

        assert_eq!(player.get_location(), (5, 5));
        player.update(&no_press);
        assert_eq!(player.get_location(), (4, 5));
        player.update(&no_press);
        assert_eq!(player.get_location(), (3, 5));
    }

    #[test]
    fn player_turns_up() {
        let mut player = Player::new((5, 5), &cell_size);

        assert_eq!(player.get_location(), (5, 5));
        player.update(&no_press);
        assert_eq!(player.get_location(), (4, 5));
        player.update(&up_press);
        assert_eq!(player.get_location(), (4, 4));
        player.update(&no_press);
        assert_eq!(player.get_location(), (4, 3));
    }

    #[test]
    fn player_cannot_180() {
        let mut player = Player::new((5, 5), &cell_size); // Player starts going left

        assert_eq!(player.get_location(), (5, 5));
        player.update(&no_press);
        assert_eq!(player.get_location(), (4, 5));
        player.update(&right_press); // Player receives right command
        assert_eq!(player.get_location(), (3, 5));
        player.update(&no_press); // Right command is ignored, left continues
        assert_eq!(player.get_location(), (2, 5));
    }

    #[test]
    fn player_warparound_left() {
        let mut player = Player::new((1, 1), &cell_size);

        assert_eq!(player.get_location(), (1, 1));
        player.update(&no_press);
        assert_eq!(player.get_location(), (0, 1));
        player.update(&no_press);
        assert_eq!(player.get_location(), (5, 1));
    }

    #[test]
    fn player_warparound_down() {
        let mut player = Player::new((4, 4), &cell_size);

        assert_eq!(player.get_location(), (4, 4));
        player.update(&down_press);
        assert_eq!(player.get_location(), (4, 5));
        player.update(&down_press);
        assert_eq!(player.get_location(), (4, 0));
    }

    #[test]
    fn player_warparound_right() {
        let mut player = Player::new((4, 4), &cell_size);

        assert_eq!(player.get_location(), (4, 4));
        player.update(&down_press);
        assert_eq!(player.get_location(), (4, 5));
        player.update(&right_press);
        assert_eq!(player.get_location(), (5, 5));
        player.update(&no_press);
        assert_eq!(player.get_location(), (0, 5));
    }

    #[test]
    fn player_warparound_up() {
        let mut player = Player::new((1, 1), &cell_size);

        assert_eq!(player.get_location(), (1, 1));
        player.update(&up_press);
        assert_eq!(player.get_location(), (1, 0));
        player.update(&no_press);
        assert_eq!(player.get_location(), (1, 5));
    }
}
