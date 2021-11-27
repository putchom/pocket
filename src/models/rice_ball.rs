pub struct RiceBall {
    pub amount: i32,
    pub last_step_count: i32,
}

impl RiceBall {
    pub fn new() -> RiceBall {
        RiceBall {
            amount: 0,
            last_step_count: 0,
        }
    }
}