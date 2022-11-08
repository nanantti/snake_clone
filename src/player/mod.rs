use super::MoveKeys;
mod body;
mod head;

pub struct Player<'a> {
    head: head::Head<'a>,
}

impl Player<'_> {
    pub fn new(initial_location: (i32, i32), n_cells: &(i32, i32)) -> Player {
        Player {
            head: head::Head::new(initial_location, n_cells),
        }
    }

    pub fn update(&mut self, keys: &MoveKeys) {
        self.head.update(keys);
    }

    pub fn get_head_location(&self) -> (i32, i32) {
        self.head.get_location()
    }
}
