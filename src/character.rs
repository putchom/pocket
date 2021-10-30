use embedded_graphics::{image::ImageRawLE, pixelcolor::Rgb565, prelude::*};

pub enum CharacterState {
    Happy,
    Angry,
}

pub struct Character {
    pub state: CharacterState,
}

impl Character {
    pub fn new(state: CharacterState) -> Character {
        Character { state }
    }
    pub fn get_image_data(&self) -> ImageRawLE<'static, Rgb565> {
        const WIDTH: u32 = 86;
        const HEIGHT: u32 = 64;

        match self.state {
            CharacterState::Happy => {
                ImageRawLE::new(include_bytes!("./assets/happy.raw"), WIDTH, HEIGHT)
            }
            CharacterState::Angry => {
                ImageRawLE::new(include_bytes!("./assets/angry.raw"), WIDTH, HEIGHT)
            }
        }
    }
    pub fn get_point(&self) -> Point {
        match self.state {
            CharacterState::Happy => Point::new(0, 32),
            CharacterState::Angry => Point::new(0, 32),
        }
    }
}
