pub struct Shuriken {
    pub amount: i32,
    pub last_step_count: i32,
}

impl Default for Shuriken {
    fn default() -> Self {
        Shuriken::new()
    }
}

impl Shuriken {
    pub fn new() -> Shuriken {
        Shuriken {
            amount: 0,
            last_step_count: 0,
        }
    }
}