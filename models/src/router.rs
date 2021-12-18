#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Route {
    Home,
    Meal,
    Play,
    Game,
}

pub struct Router {
    pub route: Route,
}

impl Router {
    pub fn new(route: Route) -> Router {
        Router { route }
    }
    pub fn update(&mut self, route: Route) {
        self.route = route
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut router = Router { route: Route::Home };

        Router::update(&mut router, Route::Meal);

        assert_eq!(router.route, Route::Meal);
    }
}