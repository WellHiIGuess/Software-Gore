use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::{Texture2D};

pub unsafe fn draw(d: &mut RaylibDrawHandle, page: String, x: i32, y: i32, search_texture: &Texture2D, cur_frame: &Texture2D, video_downloaded: bool) {
    if page == "home" {
        d.draw_texture(search_texture,x + 20, y + 50, Color::WHITE);
        d.draw_text("BooTube", x + 30, y + 120, 24, Color::BLUE);

        if video_downloaded {
            d.draw_text("File Converters", x + 30, y + 149, 24, Color::BLUE);
        }
    } else if page == "BooTube" {
        d.draw_texture_ex(cur_frame, Vector2::new((x + 35) as f32, (y + 35) as f32), 0.0, 1.6 as f32, Color::WHITE);
        d.draw_rectangle(x + 35, y + 375, 120, 24, Color::LIGHTGRAY);
        d.draw_text("download", x + 41, y + 376, 24, Color::GRAY);
    } else if page == "file converters" {
        d. draw_text("mp4 converter", x + 10, y + 10, 24, Color::BLUE);

        d.draw_text("EPIC mp4 converter", x + 10, y + 34, 24, Color::BLUE);
    } else if page == "mp4 converter" {
        // d.draw_rectangle(x + )
    } else if page == "EPIC mp4 converter" {

    }
}
