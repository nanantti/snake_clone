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

    pub fn update(&mut self, keys: &MoveKeys) {
        self.head.update(keys);
    }

    pub fn get_head_location(&self) -> (i32, i32) {
        self.head.get_location()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CELL_SIZE: (i32, i32) = (5, 5);

    #[test]
    fn snake_moves_to_left() {
        let mut player = Player::new((0, 0), &CELL_SIZE);
        assert_eq! {player.get_head_location(), (0,0)}
        //assert_eq!{player.body.sections.len(), 0}
    }
}
