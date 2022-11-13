use super::engine;

pub struct FruitGenerator<'a> {
    number_of_cells: &'a (i32, i32),
}

impl FruitGenerator<'_> {
    pub fn new<'a>(n_cells: &'a (i32, i32), seed: u64) -> FruitGenerator<'a> {
        engine::set_rand_seed(seed);
        FruitGenerator {
            number_of_cells: n_cells,
        }
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
    fn rolled_tile_is_inside_grid() {
        for seed in 0..999 {
            let mut gen = FruitGenerator::new(&CELL_SIZE, seed);
            let tile = gen.random_tile();

            assert! {0 <= tile.0}
            assert! {tile.0 < CELL_SIZE.0}

            assert! {0 <= tile.1}
            assert! {tile.1 < CELL_SIZE.1}
        }
    }
}
