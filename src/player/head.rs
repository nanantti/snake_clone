use super::super::MoveKeys;

pub struct Head<'a> {
    pub location: (i32, i32),
    direction: MoveDirection,
    number_of_cells: &'a (i32, i32),
}

impl Head<'_> {
    pub fn new(initial_location: (i32, i32), n_cells: &(i32, i32)) -> Head {
        Head {
            location: initial_location,
            direction: MoveDirection::Left,
            number_of_cells: n_cells,
        }
    }

    pub fn get_location(&self) -> (i32, i32) {
        self.location
    }

    pub fn update(&mut self, keys: &MoveKeys) {
        self.update_direction(keys);
        self.update_location();
        self.check_warparound();
    }

    pub fn get_angle(&self) -> f32 {
        match self.direction {
            MoveDirection::Up => 0.0,
            MoveDirection::Down => 180.0,
            MoveDirection::Left => 270.0,
            MoveDirection::Right => 90.0,
        }
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
        } else if self.location.0 >= self.number_of_cells.0 {
            self.location.0 = 0;
        }
        if self.location.1 < 0 {
            self.location.1 = self.number_of_cells.1;
        } else if self.location.1 >= self.number_of_cells.1 {
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
    const NO_PRESS: MoveKeys = MoveKeys {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    const UP_PRESS: MoveKeys = MoveKeys {
        up: true,
        down: false,
        left: false,
        right: false,
    };

    const DOWN_PRESS: MoveKeys = MoveKeys {
        up: false,
        down: true,
        left: false,
        right: false,
    };

    const RIGHT_PRESS: MoveKeys = MoveKeys {
        up: false,
        down: false,
        left: false,
        right: true,
    };

    const CELL_SIZE: (i32, i32) = (5, 5);

    #[test]
    fn head_starts_moving_left() {
        let mut head = Head::new((4, 4), &CELL_SIZE);

        assert_eq!(head.get_location(), (4, 4));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (3, 4));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (2, 4));
    }

    #[test]
    fn head_turns_up() {
        let mut head = Head::new((4, 4), &CELL_SIZE);

        assert_eq!(head.get_location(), (4, 4));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (3, 4));
        head.update(&UP_PRESS);
        assert_eq!(head.get_location(), (3, 3));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (3, 2));
    }

    #[test]
    fn head_cannot_180() {
        let mut head = Head::new((4, 4), &CELL_SIZE); // Head starts going left

        assert_eq!(head.get_location(), (4, 4));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (3, 4));
        head.update(&RIGHT_PRESS); // Head receives right command
        assert_eq!(head.get_location(), (2, 4));
        head.update(&NO_PRESS); // Right command is ignored, left continues
        assert_eq!(head.get_location(), (1, 4));
    }

    #[test]
    fn head_warparound_left() {
        let mut head = Head::new((1, 1), &CELL_SIZE);

        assert_eq!(head.get_location(), (1, 1));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (0, 1));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (5, 1));
    }

    #[test]
    fn head_warparound_down() {
        let mut head = Head::new((4, 4), &CELL_SIZE);

        assert_eq!(head.get_location(), (4, 4));
        head.update(&DOWN_PRESS);
        assert_eq!(head.get_location(), (4, 0));
        head.update(&DOWN_PRESS);
        assert_eq!(head.get_location(), (4, 1));
    }

    #[test]
    fn head_warparound_right() {
        let mut head = Head::new((4, 4), &CELL_SIZE);

        assert_eq!(head.get_location(), (4, 4));
        head.update(&UP_PRESS);
        assert_eq!(head.get_location(), (4, 3));
        head.update(&RIGHT_PRESS);
        assert_eq!(head.get_location(), (0, 3));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (1, 3));
    }

    #[test]
    fn head_warparound_up() {
        let mut head = Head::new((1, 1), &CELL_SIZE);

        assert_eq!(head.get_location(), (1, 1));
        head.update(&UP_PRESS);
        assert_eq!(head.get_location(), (1, 0));
        head.update(&NO_PRESS);
        assert_eq!(head.get_location(), (1, 5));
    }

    #[test]
    fn head_angle() {
        let mut head = Head::new((1, 1), &CELL_SIZE);

        assert_eq!(head.get_angle(), 270.0);
        head.update(&UP_PRESS);
        assert_eq!(head.get_angle(), 0.0);
        head.update(&RIGHT_PRESS);
        assert_eq!(head.get_angle(), 90.0);
        head.update(&DOWN_PRESS);
        assert_eq!(head.get_angle(), 180.0);
    }
}
