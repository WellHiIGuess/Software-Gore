use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::Texture2D;
use raylib::ffi::KeyboardKey;

#[allow(non_upper_case_globals)]
static mut i: i32 = 0;
#[allow(non_upper_case_globals)]
static mut mi: i32 = 0;
static mut HYPE: i32 = 0;
static mut CHAT: String = String::new();

pub unsafe fn draw(d: &mut RaylibDrawHandle, x: i32, y: i32, friend_profile_texture: &Texture2D) {
    let friends_user = "owomega";

    if i == 0 {
        let message = ": Hey man hyd";
        let j = friends_user.to_owned() + message;
        CHAT = j;
    }

    d.draw_rectangle(x, y, 200, 670, Color::new(40, 40, 65, 255));
    d.draw_texture(friend_profile_texture, x + 5, y + 5, Color::WHITE);
    d.draw_text(friends_user, x + 45, y + 10, 24, Color::WHITE);

    // chat bar
    d.draw_rectangle(x + 200, y + 570, 800, 100, Color::new(30, 30, 55, 255));

    // upload file
    d.draw_rectangle(x + 905, y + 575, 90, 90, Color::GRAY);

    // chat
    d.draw_text(CHAT.as_str(), x + 205, y + 540 - (35 * (i + mi)), 24, Color::WHITE);

    match i {
        0 => d.draw_text("1. Not much i just saw this funny clip\n2. I just saw this supper funny clip!", x + 200, y + 570, 25, Color::WHITE),
        _ => {}
    }
}

fn message_wait() {
    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(500);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    assert!(now.elapsed() >= ten_millis);
}

pub unsafe fn update(d: &mut RaylibDrawHandle) {
    let friends_user = "owomega";

    if d.is_key_pressed(KeyboardKey::KEY_ONE) {
        if i == 0 {
            let message = "\nme: Not much i just saw this funny clip\n";
            let j = CHAT.to_owned() + message;
            CHAT = j;
            i += 1;
            mi += 1;

            message_wait();

            let message = &(friends_user.to_owned() + ": oh cool send it to me.\n") as &str;
            CHAT = CHAT.to_owned() + &message;
        }
    } else if d.is_key_pressed(KeyboardKey::KEY_TWO) {
        if i == 0 {
            let message = "\nme: Bro I just saw this supper funny clip!\n";
            CHAT = CHAT.to_owned() + message;
            i += 1;
            mi += 1;
            HYPE += 1;

            message_wait();

            let message = &(friends_user.to_owned() + ": oh cool send it to me.\n") as &str;
            CHAT = CHAT.to_owned() + &message;
        }
    }
}
