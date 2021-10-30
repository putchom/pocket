pub enum CharacterState {
    Happy,
    Angry,
}

pub struct Character {
    pub state: CharacterState,
}

impl Character {
    pub fn new() -> Character {
        Character {
            state: CharacterState::Happy,
        }
    }
}
