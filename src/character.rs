use embedded_graphics::{image::ImageRawLE, pixelcolor::Rgb565, prelude::*};

pub enum CharacterState {
    Angry,
    Away,
    Eat,
    Happy,
    Play,
    Shy,
    Sleep,
}

pub struct Character {
    pub state: CharacterState,
}

impl Character {
    pub fn new(state: CharacterState) -> Character {
        Character { state }
    }
    pub fn get_image_data(&self) -> ImageRawLE<'static, Rgb565> {
        const WIDTH: u32 = 180;
        const HEIGHT: u32 = 117;

        let data;

        match self.state {
            CharacterState::Angry => data = include_bytes!("./assets/character/angry/default.raw"),
            CharacterState::Away => data = include_bytes!("./assets/character/away/default.raw"),
            CharacterState::Eat => data = include_bytes!("./assets/character/eat/default.raw"),
            CharacterState::Happy => data = include_bytes!("./assets/character/happy/default.raw"),
            CharacterState::Play => data = include_bytes!("./assets/character/play/default.raw"),
            CharacterState::Shy => data = include_bytes!("./assets/character/shy/default.raw"),
            CharacterState::Sleep => data = include_bytes!("./assets/character/sleep/default.raw"),
        }

        ImageRawLE::new(data, WIDTH, HEIGHT)
    }
    pub fn get_point(&self) -> Point {
        match self.state {
            CharacterState::Angry => Point::new(60, 90),
            CharacterState::Away => Point::new(60, 90),
            CharacterState::Eat => Point::new(0, 90),
            CharacterState::Happy => Point::new(60, 90),
            CharacterState::Play => Point::new(0, 90),
            CharacterState::Shy => Point::new(60, 90),
            CharacterState::Sleep => Point::new(60, 90),
        }
    }
}
