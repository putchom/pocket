pub enum NavigationFocus {
    Home,
    Clock,
    Eat,
}

pub struct Navigation {
    pub focus: NavigationFocus,
}

impl Navigation {
    pub fn new(focus: NavigationFocus) -> Navigation {
        Navigation { focus }
    }
}
