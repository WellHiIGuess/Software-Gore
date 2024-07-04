/*
            T o d o  l i s t o

    C o d i n g
    TODO: make the file converter websites

    A r t
    TODO: make BooTube look more like youtube
    TODO: Home button sprite

    S o u n d
    TODO: click sound
    TODO: keyboard sound

    T h e  I m p o s i b l e
    TODO: Finish The Game
*/

mod application;
mod discord;
mod browser;
mod finder;

use raylib::prelude::*;
use raylib::consts::MouseButton;
use crate::application::Application;

fn notification(d: &mut RaylibDrawHandle, friend_profile_texture: &Texture2D, not_frames: i32) {
    d.draw_rectangle(0, 0, 300, 70, Color::new(0, 0, 25, 255).fade(not_frames as f32 / 50.0));
    d.draw_text("Message from", 0, 0, 27, Color::WHITE.fade(not_frames as f32 / 50.0));
    d.draw_text("owomega", 0, 27, 27, Color::WHITE.fade(not_frames as f32 / 50.0));
    d.draw_texture_ex(friend_profile_texture, Vector2::new(210.0, 5.0), 0.0, 2.0, Color::WHITE.fade(not_frames as f32 / 50.0));
}

fn main() {
    let (mut rl, thread) = init()
        .title("software gore")
        .resizable()
        .build();

    rl.set_target_fps(60);

    let background = rl.load_texture(&thread, "src/images/Sprite-0001.png");

    let search_texture = rl.load_texture(&thread, "src/images/search.png");

    // for desktop
    let browser_texture = rl.load_texture(&thread, "src/images/browser.png");
    let mut browser_x;
    let mut browser_y;
    let mut video_frame: usize = 0;
    let mut prolong = 0;
    let mut cur_page = "BooTube";

    let discord_texture = rl.load_texture(&thread, "src/images/discord.png");
    let friend_profile_texture = rl.load_texture(&thread, "src/images/profile.png");
    let friend_profile_texture_copy = rl.load_texture(&thread, "src/images/profile.png");
    let mut discord_x;
    let mut discord_y;

    let finder_texture = rl.load_texture(&thread, "src/images/finder.png");
    let mut finder_x;
    let mut finder_y;
    let mut video_downloaded = false;

    let mut dragging= false;
    let mut being_dragged: usize = 0;
    let mut d_distance_x: i32 = 0;
    let mut d_distance_y: i32 = 0;

    // notification variables
    let mut notified = false;
    let mut wait_frames = 200;
    let mut not_frames = 200;
    let mut buffer_frames = 50;

    let mut apps = [
        Application::new(1000, 700, Color::new(0, 0, 25, 255)),
        Application::new(1200, 700, Color::new(255, 255, 255, 255)),
        Application::new(700, 500, Color::new(255, 255, 255, 255))
    ];

    let mut app_order = [
        0,
        1,
        2
    ];

    apps[1].y = 100;

    let video_frames = [
        rl.load_texture(&thread, "src/images/video/Video1.png"),
        rl.load_texture(&thread, "src/images/video/Video2.png"),
        rl.load_texture(&thread, "src/images/video/Video3.png"),
        rl.load_texture(&thread, "src/images/video/Video4.png"),
        rl.load_texture(&thread, "src/images/video/Video5.png"),
        rl.load_texture(&thread, "src/images/video/Video6.png"),
        rl.load_texture(&thread, "src/images/video/Video7.png"),
        rl.load_texture(&thread, "src/images/video/Video8.png"),
        rl.load_texture(&thread, "src/images/video/Video9.png"),
        rl.load_texture(&thread, "src/images/video/Video10.png"),
        rl.load_texture(&thread, "src/images/video/Video11.png"),
        rl.load_texture(&thread, "src/images/video/Video12.png"),
        rl.load_texture(&thread, "src/images/video/Video13.png"),
        rl.load_texture(&thread, "src/images/video/Video14.png"),
        rl.load_texture(&thread, "src/images/video/Video15.png"),
        rl.load_texture(&thread, "src/images/video/Video16.png"),
        rl.load_texture(&thread, "src/images/video/Video17.png"),
        rl.load_texture(&thread, "src/images/video/Video18.png"),
        rl.load_texture(&thread, "src/images/video/Video19.png"),
        rl.load_texture(&thread, "src/images/video/Video20.png"),
    ];

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);


        discord_x = d.get_screen_width() / 2;
        discord_y = d.get_screen_height() - 42;

        browser_x = discord_x - 45;
        browser_y = discord_y;

        finder_x = discord_x + 45;
        finder_y = discord_y;

        d.clear_background(Color::SKYBLUE);

        d.draw_texture_ex(background.as_ref().unwrap(), Vector2::zero(), 0.0, d.get_screen_width() as f32 / 700.0, Color::WHITE);

        let mut i: usize = 0;

        while i < apps.len() {
            let j = app_order[i];

            if apps[j].opened {
                d.draw_rectangle(apps[j].x, apps[j].y, apps[j].width, apps[j].height, apps[j].color);

                unsafe {
                    if apps[0].opened && j == 0 {
                        discord::draw(&mut d, apps[0].x, apps[0].y + 30, friend_profile_texture.as_ref().unwrap());
                    }

                    if apps[1].opened && j == 1 {
                        browser::draw(&mut d, cur_page.to_owned(), apps[1].x, apps[1].y + 30,  search_texture.as_ref().unwrap(), video_frames[video_frame].as_ref().unwrap(), video_downloaded);

                        if video_frame != 19 {
                            if prolong < 100 && video_frame == 1{
                                prolong += 1;
                            } else {
                                video_frame += 1;
                            }
                        } else {
                            video_frame = 0;
                            prolong = 0;
                        }
                    }

                    if apps[2].opened && j == 2 {
                        finder::draw(&mut d, apps[2].x, apps[2].y + 30, video_downloaded);
                    }
                }

                d.draw_rectangle(apps[j].x, apps[j].y, apps[j].width, 30, Color::new(25, 25, 25, 255));

                if j == 1 {
                    // home button
                    d.draw_rectangle(apps[j].x + 2, apps[j].y + 2, 25, 25, Color::RED);
                }
            }

            i += 1;
        }

        d.draw_rectangle(0,  d.get_screen_height() - 50, get_monitor_width(get_current_monitor()), 50, Color::DARKGRAY.fade(0.95));
        d.draw_text("9:05", d.get_screen_width() - 70, d.get_screen_height() - 35, 25, Color::WHITE);
        d.draw_texture(discord_texture.as_ref().unwrap(), discord_x, discord_y, Color::WHITE);
        d.draw_texture(browser_texture.as_ref().unwrap(), browser_x, browser_y, Color::WHITE);
        d.draw_texture(finder_texture.as_ref().unwrap(), finder_x, finder_y, Color::WHITE);


        // notification
        if wait_frames != 0 && apps[1].opened {
            wait_frames -= 1;
        }

        if apps[1].opened && !notified && wait_frames == 0 {
            notification(&mut d, friend_profile_texture_copy.as_ref().unwrap(), not_frames);
            if not_frames != 0 {
                not_frames -= 1;
            }
        }

        if not_frames == 0 {
            buffer_frames -= 1;
        }

        if buffer_frames == 0 {
            notified = true;
        }


        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            // icons
            if d.get_mouse_x() >= discord_x && d.get_mouse_y() >= discord_y && d.get_mouse_x() < discord_x + 35 && d.get_mouse_y() < discord_y + 35 {
                apps[0].opened = !apps[0].opened;
            }

            if d.get_mouse_x() >= browser_x && d.get_mouse_y() >= browser_y && d.get_mouse_x() < browser_x + 35 && d.get_mouse_y() < browser_y + 35 {
                apps[1].opened = !apps[1].opened;
            }

            if d.get_mouse_x() >= finder_x && d.get_mouse_y() >= finder_y && d.get_mouse_x() < finder_x + 35 && d.get_mouse_y() < finder_y + 35 {
                apps[2].opened = !apps[2].opened;
            }

            // application bars
            if d.get_mouse_x() >= apps[0].x && d.get_mouse_y() >= apps[0].y && d.get_mouse_x() < apps[0].x + apps[0].width && d.get_mouse_y() < apps[0].y + 30 && apps[0].opened {
                being_dragged = 0;
                dragging = true;

                d_distance_x = d.get_mouse_x() - apps[being_dragged].x;
                d_distance_y = d.get_mouse_y() - apps[being_dragged].y;

                let temp = app_order[app_order.len() - 1];
                let o_index = app_order.iter().position(|&r| r == 0).unwrap();
                app_order[app_order.len() - 1] = app_order[o_index];
                app_order[o_index] = temp;
            }

            if d.get_mouse_x() >= apps[1].x && d.get_mouse_y() >= apps[1].y && d.get_mouse_x() < apps[1].x + apps[1].width && d.get_mouse_y() < apps[1].y + 30 && apps[1].opened {
                being_dragged = 1;
                dragging = true;

                d_distance_x = d.get_mouse_x() - apps[being_dragged].x;
                d_distance_y = d.get_mouse_y() - apps[being_dragged].y;

                let temp = app_order[app_order.len() - 1];
                let o_index = app_order.iter().position(|&r| r == 1).unwrap();
                app_order[app_order.len() - 1] = app_order[o_index];
                app_order[o_index] = temp;
            }

            if d.get_mouse_x() >= apps[2].x && d.get_mouse_y() >= apps[2].y && d.get_mouse_x() < apps[2].x + apps[2].width && d.get_mouse_y() < apps[2].y + 30 && apps[2].opened {
                being_dragged = 2;
                dragging = true;

                d_distance_x = d.get_mouse_x() - apps[being_dragged].x;
                d_distance_y = d.get_mouse_y() - apps[being_dragged].y;

                let temp = app_order[app_order.len() - 1];
                let o_index = app_order.iter().position(|&r| r == 2).unwrap();
                app_order[app_order.len() - 1] = app_order[o_index];
                app_order[o_index] = temp;
            }

            // home button for browser
            if d.get_mouse_x() >= apps[1].x + 2 && d.get_mouse_y() >= apps[1].y + 2 && d.get_mouse_x() <= apps[1].x + 27 && d.get_mouse_y() <= apps[1].y + 27 {
                cur_page = "home";
            }

            // links (remember to always make the y + 30 of what it actually is for >= and <=)
            if cur_page == "home" {
                if d.get_mouse_x() >= apps[1].x + 30 && d.get_mouse_y() >= apps[1].y + 150 && d.get_mouse_x() <= apps[1].x + 134 && d.get_mouse_y() <= apps[1].y + 174 {
                    cur_page = "BooTube";
                }
            }

            if cur_page == "home" && video_downloaded {
                if d.get_mouse_x() >= apps[1].x + 30 && d.get_mouse_y() >= apps[1].y + 179 && d.get_mouse_x() <= apps[1].x + 174 && d.get_mouse_y() <= apps[1].y + 203 {
                    cur_page = "file converters";
                }
            }

            if cur_page == "file converters" && video_downloaded {
                if d.get_mouse_x() >= apps[1].x + 10 && d.get_mouse_y() >= apps[1].y + 30  && d.get_mouse_x() <= apps[1].x + 170 && d.get_mouse_y() <= apps[1].y + 54 {
                    cur_page = "home";
                }
            }

            // download button
            if cur_page == "BooTube" {
                if d.get_mouse_x() >= apps[1].x + 35 && d.get_mouse_y() >= apps[1].y + 405 && d.get_mouse_x() <= apps[1].x + 155 && d.get_mouse_y() <= apps[1].y + 429 {
                    video_downloaded = true;
                    apps[2].opened = true;
                }
            }
        }

        // dragging
        if dragging {
            apps[being_dragged].x = d.get_mouse_x() - d_distance_x;
            apps[being_dragged].y = d.get_mouse_y() - d_distance_y;
        }

        if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            dragging = false;
        }

        // update functions
        if apps[0].opened {
            unsafe {
                discord::update(&mut d);
            }
        }
    }
}
// WHY THE FUCK IS THIS OVER 250 LINES
// it's over 300 lines now and I wanna die
