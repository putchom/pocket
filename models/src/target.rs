pub enum TargetPosition {
    Left,
    Center,
    Right,
}

pub struct Target {
    pub position: TargetPosition
}

impl Default for Target {
    fn default() -> Self {
        Target::new()
    }
}

impl Target {
    pub fn new() -> Target {
        Target {
            position: TargetPosition::Left
        }
    }
    pub fn update(&mut self) {
        match self.position {
            TargetPosition::Left => {
                self.position = TargetPosition::Center
            }
            TargetPosition::Center => {
                self.position = TargetPosition::Right
            }
            TargetPosition::Right => {
                self.position = TargetPosition::Left
            }
        }
    }
}