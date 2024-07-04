use raylib::color::Color;

#[derive(Copy, Clone)]
pub struct Application {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub opened: bool,
    pub color: Color
}

impl Application {
    pub fn new(width: i32, height: i32, color: Color) -> Self {
        Self{
            x: 0,
            y: 0,
            width,
            height,
            opened: false,
            color
        }
    }
}
