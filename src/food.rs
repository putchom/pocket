pub struct Food {
    pub amount: i32,
}

impl Food {
    pub fn new(amount: i32) -> Food {
        Food {
            amount
        }
    }
    pub fn increase(&mut self, max: i32) {
        if self.amount < max {
            self.amount += 1;
        }
    }
    pub fn decrease(&mut self) {
        if self.amount > 0 {
            self.amount -= 1;
        }
    }
}