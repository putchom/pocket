#[derive(Clone, Copy, PartialEq)]
pub enum Route {
    Home,
    Food,
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
