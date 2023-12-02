extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use sdl2::rect::Rect;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Melvor Idle Budget", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let button1 = Rect::new(200, 300, 200, 100);
    let mut leftDown = false;
    let mut button1Down = false;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(2, 64, 255));
        canvas.clear();
 
        canvas.set_draw_color(Color::RGB(20, 20, 10));
        canvas.fill_rect(button1).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::MouseButtonDown{x, y, mouse_btn: MouseButton::Left, ..} => {
                    leftDown = true;
                    if button1.contains_point((x, y)){
                        button1Down = true;
                    }
                    else {
                        button1Down = false;
                    }
                }
                Event::MouseButtonUp {mouse_btn: MouseButton::Left, ..} => {
                    leftDown = false;
                }
                _ => {}
            }
        }

        if leftDown && button1Down {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.fill_rect(button1).unwrap();
        } else {
            canvas.set_draw_color(Color::RGB(20, 20, 10)); 
            canvas.fill_rect(button1).unwrap();
        }

        let state = event_pump.mouse_state();

        let buttons:Vec<_> = state.pressed_mouse_buttons().collect();

        if !buttons.is_empty(){
            println!("{:?}",buttons);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
