use super::engine;
use super::grid;
use super::MoveKeys;
mod body;
mod head;

pub struct Player<'a> {
    head: head::Head<'a>,
    body: body::Body,
}

impl Player<'_> {
    pub fn new(initial_location: (i32, i32), n_cells: &(i32, i32)) -> Player {
        Player {
            head: head::Head::new(initial_location, n_cells),
            body: body::Body::new(),
        }
    }

    pub fn update(&mut self, keys: &MoveKeys, grow: bool) {
        self.body.add(self.head.get_location());
        self.head.update(keys);
        if !grow {
            self.body.drop_last();
        }
    }

    pub fn get_head_location(&self) -> (i32, i32) {
        self.head.get_location()
    }

    pub fn draw(&self, gd: &grid::Grid) {
        self.draw_head(gd);
        self.draw_body(gd);
    }

    pub fn collision(&self, tile: &(i32, i32)) -> bool {
        if *tile == self.head.get_location() {
            return true;
        }
        return self.body.collision(tile);
    }

    fn draw_head(&self, gd: &grid::Grid) {
        let center = gd.get_cell_center(self.get_head_location());
        engine::draw_triangle(
            center.0,
            center.1,
            gd.get_cell_size(),
            self.head.get_angle(),
            engine::SNAKE_COLOR,
        );
    }

    fn draw_body(&self, gd: &grid::Grid) {
        for section in self.body.get_locations() {
            self.draw_circle_in_tile(*section, gd);
        }
    }

    fn draw_circle_in_tile(&self, tile: (i32, i32), gd: &grid::Grid) {
        let center = gd.get_cell_center(tile);
        engine::draw_circle(
            center.0,
            center.1,
            gd.get_cell_size() * 0.50,
            engine::SNAKE_COLOR,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const NO_PRESS: MoveKeys = MoveKeys {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    const CELL_SIZE: (i32, i32) = (5, 5);

    #[test]
    fn snake_moves_to_left_and_grows() {
        let mut player = Player::new((5, 0), &CELL_SIZE);
        assert_eq! {player.get_head_location(), (5, 0)};

        player.update(&NO_PRESS, true);
        assert_eq! {player.get_head_location(), (4, 0)};
        assert_eq! {player.body.collision(&(5 ,0)), true};

        player.update(&NO_PRESS, true);
        assert_eq! {player.get_head_location(), (3, 0)};
        assert_eq! {player.body.collision(&(4 ,0)), true};
        assert_eq! {player.body.collision(&(5 ,0)), true};

        player.update(&NO_PRESS, false);
        assert_eq! {player.get_head_location(), (2, 0)};
        assert_eq! {player.body.collision(&(3 ,0)), true};
        assert_eq! {player.body.collision(&(4 ,0)), true};
        assert_eq! {player.body.collision(&(5 ,0)), false};
    }

    #[test]
    fn player_collision() {
        let mut player = Player::new((5, 0), &CELL_SIZE);
        assert_eq! {player.collision(&(5, 0)), true};
        assert_eq! {player.collision(&(4, 0)), false};
        assert_eq! {player.collision(&(3, 0)), false};

        player.update(&NO_PRESS, true);
        assert_eq! {player.collision(&(5, 0)), true};
        assert_eq! {player.collision(&(4, 0)), true};
        assert_eq! {player.collision(&(3, 0)), false};
    }
}
