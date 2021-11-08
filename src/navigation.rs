use crate::router::Route;

pub enum Direction {
    Left,
    Right,
}

pub struct Navigation {
    pub focus: Route,
}

impl Navigation {
    pub fn new(focus: Route) -> Navigation {
        Navigation { focus }
    }
    pub fn update(&mut self, direction: Direction) {
        match self.focus {
            Route::Home => match direction {
                Direction::Left => {}
                Direction::Right => self.focus = Route::Clock,
            },
            Route::Clock => match direction {
                Direction::Left => self.focus = Route::Home,
                Direction::Right => self.focus = Route::Eat,
            },
            Route::Eat => match direction {
                Direction::Left => self.focus = Route::Clock,
                Direction::Right => {}
            },
        }
    }
}
