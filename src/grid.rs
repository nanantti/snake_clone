pub struct Grid {
    number_of_cells: (i32, i32),
    screen_size: (f32, f32),
}

impl Grid {
    pub fn get_cell_center(&self, cell_indexes: (i32, i32)) -> (f32, f32) {
        (self.cell_center_coordinate(cell_indexes.0, self.number_of_cells.0, self.screen_size.0),
        self.cell_center_coordinate(cell_indexes.1, self.number_of_cells.1, self.screen_size.1))
    }

    fn cell_center_coordinate(&self, index: i32, total_num_cells: i32, total_size: f32) -> f32 {
        (total_size / (total_num_cells as f32)) * (0.50 + (index as f32))
    }
}

#[cfg(test)]
mod tests_grid {
    use super::*;

    #[test]
    fn cell_center_coordinates() {
        let grid = Grid {
            number_of_cells: (3, 3),
            screen_size: (300.0, 300.0),
        };
        assert_eq!{grid.get_cell_center((0, 0)), (50.0, 50.0)}
        assert_eq!{grid.get_cell_center((1, 0)), (150.0, 50.0)}
        assert_eq!{grid.get_cell_center((0, 1)), (50.0, 150.0)}
        assert_eq!{grid.get_cell_center((2, 2)), (250.0, 250.0)}
    }
}
