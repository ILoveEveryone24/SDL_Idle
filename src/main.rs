extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::image::{InitFlag, LoadTexture};

struct panel {
    button: Rect,
    line: Rect,
    overline: Rect,
    loot: Rect
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let image_context = sdl2::image::init(InitFlag::PNG).unwrap();

    let window = video_subsystem.window("Melvor Idle Budget", 2560, 1600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let button1 = Rect::new(300, 300, 200, 100);
    let mut button1Down = false;

    let loot = Rect::new(300, 10, 200, 200);

    let line = Rect::new(150, 220, 500, 50);
    let mut overline = Rect::new(150, 220, 0, 50);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("../pictures/bonfire.png").expect("Failed to load PNG");

    'running: loop {
        canvas.set_draw_color(Color::RGB(66, 75, 79));
        canvas.clear();
        
        canvas.copy(&texture, None, loot);

        canvas.set_draw_color(Color::RGB(88, 97,101));
        canvas.fill_rect(line).unwrap();
 
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::MouseButtonDown{x, y, mouse_btn: MouseButton::Left, ..} => {
                    if button1.contains_point((x, y)){
                        button1Down = !button1Down;
                    }
                }
                _ => {}
            }
        }

        if button1Down {
            canvas.set_draw_color(Color::RGB(215, 120, 192));
            canvas.fill_rect(button1).unwrap();
           
            if overline.width() > line.width() {
                overline.set_width(0);
            } 
            else{ 
                overline.set_width(overline.width() + 10);
            }
            canvas.set_draw_color(Color::RGB(255, 234, 139));
            canvas.fill_rect(overline).unwrap();
        } else {
            canvas.set_draw_color(Color::RGB(121, 215, 120)); 
            canvas.fill_rect(button1).unwrap();
            overline.set_width(0);
        }

        let state = event_pump.mouse_state();

        let buttons:Vec<_> = state.pressed_mouse_buttons().collect();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
