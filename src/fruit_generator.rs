use super::engine;

pub struct FruitGenerator<'a> {
    number_of_cells: &'a (i32, i32),
}

impl FruitGenerator<'_> {
    pub fn new<'a>(n_cells: &'a (i32, i32)) -> FruitGenerator<'a> {
        FruitGenerator {
            number_of_cells: n_cells,
        }
    }

    pub fn set_rand_seed(&mut self, seed: u64) {
        engine::set_rand_seed(seed);
    }

    pub fn random_tile(&mut self) -> (i32, i32) {
        let a: i32 = engine::gen_range(0, self.number_of_cells.0);
        let b: i32 = engine::gen_range(0, self.number_of_cells.1);
        (a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CELL_SIZE: (i32, i32) = (3, 3);

    #[test]
    fn roll_fruit_location() {
        let mut gen = FruitGenerator::new(&CELL_SIZE);
        gen.set_rand_seed(0);
        assert_eq! {gen.random_tile(), (2, 1)}
        assert_eq! {gen.random_tile(), (1, 2)}
        assert_eq! {gen.random_tile(), (2, 1)}
        assert_eq! {gen.random_tile(), (0, 0)}
    }

    #[test]
    fn rolled_tile_is_inside_grid() {
        let mut gen = FruitGenerator::new(&CELL_SIZE);
        for seed in 0..999 {
            gen.set_rand_seed(seed);
            let tile = gen.random_tile();

            assert! {0 <= tile.0}
            assert! {tile.0 < CELL_SIZE.0}

            assert! {0 <= tile.1}
            assert! {tile.1 < CELL_SIZE.1}
        }
    }
}
