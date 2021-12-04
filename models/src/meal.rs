pub struct Meal {
    pub amount: i32,
}

impl Default for Meal {
    fn default() -> Self {
        Meal::new()
    }
}

impl Meal {
    pub fn new() -> Meal {
        Meal {
            amount: 0
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