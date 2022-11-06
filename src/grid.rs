pub struct Grid {
    pub number_of_cells: (i32, i32),
    pub screen_size: (f32, f32),
}

impl Grid {
    pub fn get_cell_center(&self, cell_indexes: (i32, i32)) -> (f32, f32) {
        (self.cell_center_coordinate(cell_indexes.0, self.number_of_cells.0, self.screen_size.0),
        self.cell_center_coordinate(cell_indexes.1, self.number_of_cells.1, self.screen_size.1))
    }

    pub fn get_cell_size(&self) -> f32 {
        let cell_x = self.screen_size.0 / self.number_of_cells.0 as f32;
        let cell_y = self.screen_size.1 / self.number_of_cells.1 as f32;
        if cell_x > cell_y {
            return cell_y;
        }
        cell_x
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
        assert_eq!{grid.get_cell_size(), (100.0)}
    }

    #[test]
    fn cell_size_gets_minimum_across_axis_y() {
        let grid = Grid {
            number_of_cells: (3, 3),
            screen_size: (300.0, 3000.0),
        }; // cell sizes: 100 and 1000
        assert_eq!{grid.get_cell_size(), (100.0)}
    }

    #[test]
    fn cell_size_gets_minimum_across_axis_x() {
        let grid = Grid {
            number_of_cells: (3, 3),
            screen_size: (3000.0, 300.0),
        }; // cell sizes: 1000 and 100
        assert_eq!{grid.get_cell_size(), (100.0)}
    }
}