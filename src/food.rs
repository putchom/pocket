pub struct Food {
    pub min: i32,
    pub max: i32,
    pub value: i32,
}

impl Food {
    pub fn new(max: i32, value: i32) -> Food {
        Food {
            min: 0,
            max,
            value
        }
    }
}