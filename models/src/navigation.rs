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
            Route::Game => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_home_to_left() {
        let mut navigation = Navigation { focus: Route::Home };

        Navigation::update(&mut navigation, Direction::Left);

        assert_eq!(navigation.focus, Route::Home);
    }

    #[test]
    fn test_update_home_to_right() {
        let mut navigation = Navigation { focus: Route::Home };

        Navigation::update(&mut navigation, Direction::Right);

        assert_eq!(navigation.focus, Route::Meal);
    }

    #[test]
    fn test_update_meal_to_left() {
        let mut navigation = Navigation { focus: Route::Meal };

        Navigation::update(&mut navigation, Direction::Left);

        assert_eq!(navigation.focus, Route::Home);
    }

    #[test]
    fn test_update_meal_to_right() {
        let mut navigation = Navigation { focus: Route::Meal };

        Navigation::update(&mut navigation, Direction::Right);

        assert_eq!(navigation.focus, Route::Play);
    }

    #[test]
    fn test_update_play_to_left() {
        let mut navigation = Navigation { focus: Route::Play };

        Navigation::update(&mut navigation, Direction::Left);

        assert_eq!(navigation.focus, Route::Meal);
    }

    #[test]
    fn test_update_play_to_right() {
        let mut navigation = Navigation { focus: Route::Play };

        Navigation::update(&mut navigation, Direction::Right);

        assert_eq!(navigation.focus, Route::Play);
    }
}
