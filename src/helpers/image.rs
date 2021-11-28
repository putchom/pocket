use embedded_graphics::{image::ImageRawLE, pixelcolor::Rgb565, prelude::*};

pub const WIDTH: u32 = 180;
pub const HEIGHT: u32 = 117;

#[allow(dead_code)]
pub enum CharacterState {
    Angry,
    Away,
    Eat,
    Happy,
    Play,
    Shy,
    Sleep,
}

pub fn get_character_data(state: &CharacterState) -> ImageRawLE<'static, Rgb565> {
    let data = match state {
        CharacterState::Angry => include_bytes!("../assets/character/angry/default.raw"),
        CharacterState::Away => include_bytes!("../assets/character/away/default.raw"),
        CharacterState::Eat => include_bytes!("..//assets/character/eat/default.raw"),
        CharacterState::Happy => include_bytes!("../assets/character/happy/default.raw"),
        CharacterState::Play => include_bytes!("../assets/character/play/default.raw"),
        CharacterState::Shy => include_bytes!("..//assets/character/shy/default.raw"),
        CharacterState::Sleep => include_bytes!("../assets/character/sleep/default.raw"),
    };

    ImageRawLE::new(data, WIDTH, HEIGHT)
}

pub fn get_character_point(state: &CharacterState) -> Point {
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
