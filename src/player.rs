use super::MoveKeys;

pub struct Player {
    pub location: (i32, i32),
}

impl Player {
    pub fn get_location(&self) -> (i32, i32) {
        self.location
    }

    pub fn update(&mut self, keys: &MoveKeys) {
        self.location.0 -= 1;
    }
}

#[cfg(test)]
mod tests_grid {
    use super::*;

    #[test]
    fn player_starts_moving_left() {
        let no_press = MoveKeys {
            up: false,
            down: false,
            left: false,
            right: false,
        };

        let mut player = Player { location: (5, 5) };

        assert_eq!(player.get_location(), (5, 5));
        player.update(&no_press);
        assert_eq!(player.get_location(), (4, 5));
        player.update(&no_press);
        assert_eq!(player.get_location(), (3, 5));
    }
}
