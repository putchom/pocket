use crate::models::router::Route;

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
                Direction::Right => self.focus = Route::Meal,
            },
            Route::Meal => match direction {
                Direction::Left => self.focus = Route::Home,
                Direction::Right => self.focus = Route::Play
            },
            Route::Play => match direction {
                Direction::Left => self.focus = Route::Meal,
                Direction::Right => {}
            },
        }
    }
}