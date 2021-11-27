use embedded_graphics::{image::ImageRawLE, pixelcolor::Rgb565, prelude::*};

pub const WIDTH: u32 = 180;
pub const HEIGHT: u32 = 117;

pub enum State {
    Angry,
    Away,
    Eat,
    Happy,
    Play,
    Shy,
    Sleep,
}

pub fn get_data(state: &State) -> ImageRawLE<'static, Rgb565> {
    let data = match state {
        State::Angry => include_bytes!("../assets/character/angry/default.raw"),
        State::Away => include_bytes!("../assets/character/away/default.raw"),
        State::Eat => include_bytes!("..//assets/character/eat/default.raw"),
        State::Happy => include_bytes!("../assets/character/happy/default.raw"),
        State::Play => include_bytes!("../assets/character/play/default.raw"),
        State::Shy => include_bytes!("..//assets/character/shy/default.raw"),
        State::Sleep => include_bytes!("../assets/character/sleep/default.raw"),
    };

    ImageRawLE::new(data, WIDTH, HEIGHT)
}

pub fn get_point(state: &State) -> Point {
    match state {
        State::Angry => Point::new(60, 90),
        State::Away => Point::new(60, 90),
        State::Eat => Point::new(0, 90),
        State::Happy => Point::new(60, 90),
        State::Play => Point::new(0, 90),
        State::Shy => Point::new(60, 90),
        State::Sleep => Point::new(60, 90),
    }
}
