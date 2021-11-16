pub struct Food {
    pub value: i32,
}

impl Food {
    pub fn new(value: i32) -> Food {
        Food {
            value
        }
    }
    pub fn increase(&mut self, max: i32) {
        if self.value < max {
            self.value += 1;
        }
    }

    pub fn decrease(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}