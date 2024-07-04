use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

pub fn draw(d: &mut RaylibDrawHandle, x: i32, y: i32, video_downloaded: bool) {
    d.draw_text("Downloads", x + 2, y + 5, 24, Color::GRAY);
    d.draw_rectangle(x + 124, y, 5, 471, Color::GRAY);

    if video_downloaded {
        d.draw_text("video.mp3", x + 134, y + 5, 24, Color::GRAY);
    }
}
