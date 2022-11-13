use std::collections::VecDeque;

pub struct Body {
    sections: VecDeque<(i32, i32)>,
}

impl Body {
    pub fn new() -> Body {
        Body {
            sections: VecDeque::new(),
        }
    }

    pub fn add(&mut self, new_tile: (i32, i32)) {
        self.sections.push_back(new_tile);
    }

    pub fn drop_last(&mut self) {
        self.sections.pop_front();
    }

    pub fn collision(&self, tile: &(i32, i32)) -> bool {
        for section in &self.sections {
            if tile == section {
                return true;
            }
        }
        false
    }

    pub fn get_locations(&self) -> &VecDeque<(i32, i32)> {
        &self.sections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_starts_empty() {
        let body = Body::new();
        assert_eq! {body.sections.len(), 0}
    }

    #[test]
    fn body_grows_when_added() {
        let mut body = Body::new();
        body.add((1, 1));
        assert_eq! {body.sections.len(), 1}
    }

    #[test]
    fn body_grows_in_order() {
        let mut body = Body::new();
        body.add((0, 0));
        body.add((1, 1));

        let loc = VecDeque::from([(0, 0), (1, 1)]);
        assert_eq! {body.get_locations(), &loc}
    }

    #[test]
    fn body_shrinks_when_dropped() {
        let mut body = Body::new();
        body.add((0, 0));
        body.add((1, 1));
        body.drop_last();
        let loc = VecDeque::from([(1, 1)]);
        assert_eq! {body.get_locations(), &loc}
    }

    #[test]
    fn collision_example() {
        let mut body = Body::new();
        body.add((0, 1));
        assert_eq! {body.collision(&(0, 1)), true};
        assert_eq! {body.collision(&(1, 1)), false};
    }

    #[test]
    fn if_drop_last_in_empty_body_does_nothing() {
        let mut body = Body::new();
        assert_eq! {body.sections.len(), 0}
        body.drop_last();
        assert_eq! {body.sections.len(), 0}
    }
}
