#[derive(Clone, Copy)]
pub enum Focus {
    Home,
    Clock,
    Eat,
}

pub enum Direction {
    Left,
    Right,
}

pub struct Navigation {
    pub focus: Focus,
}

impl Navigation {
    pub fn new(focus: Focus) -> Navigation {
        Navigation { focus }
    }
    pub fn update(&mut self, direction: Direction) {
        match self.focus {
            Focus::Home => match direction {
                Direction::Left => {}
                Direction::Right => self.focus = Focus::Clock,
            },
            Focus::Clock => match direction {
                Direction::Left => self.focus = Focus::Home,
                Direction::Right => self.focus = Focus::Eat,
            },
            Focus::Eat => match direction {
                Direction::Left => self.focus = Focus::Clock,
                Direction::Right => {}
            },
        }
    }
}
