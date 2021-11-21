use embedded_graphics::{image::ImageRawLE, pixelcolor::Rgb565, prelude::*};

#[derive(Clone, Copy)]
pub enum CharacterState {
    Angry,
    Away,
    Eat,
    Happy,
    Play,
    Shy,
    Sleep,
}

pub struct CharacterImage {
    pub data: ImageRawLE<'static, Rgb565>,
    pub point: Point
}

impl CharacterImage {
    pub fn new(state: CharacterState) -> CharacterImage {
        let data = Self::get_data(state);
        let point = Self::get_point(state);

        CharacterImage {
            data,
            point,
        }
    }
    fn get_data(state: CharacterState) -> ImageRawLE<'static, Rgb565> {
        const WIDTH: u32 = 180;
        const HEIGHT: u32 = 117;

        let data = match state {
            CharacterState::Angry => include_bytes!("./assets/character/angry/default.raw"),
            CharacterState::Away => include_bytes!("./assets/character/away/default.raw"),
            CharacterState::Eat => include_bytes!("./assets/character/eat/default.raw"),
            CharacterState::Happy => include_bytes!("./assets/character/happy/default.raw"),
            CharacterState::Play => include_bytes!("./assets/character/play/default.raw"),
            CharacterState::Shy => include_bytes!("./assets/character/shy/default.raw"),
            CharacterState::Sleep => include_bytes!("./assets/character/sleep/default.raw"),
        };

        ImageRawLE::new(data, WIDTH, HEIGHT)
    }
    fn get_point(state: CharacterState) -> Point {
        match state {
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
