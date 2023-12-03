extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

struct Panel {
    button: Rect,
    line: Rect,
    overline: Rect,
    loot: Rect,
    button_down: bool,
}

impl Panel {
    fn new(x: i32, y: i32) -> Panel {
        Panel {
            button: Rect::new(300 + x, 300 + y, 200, 100),
            line: Rect::new(150 + x, 220 + y, 500, 50),
            overline: Rect::new(150 + x, 220 + y, 0, 50),
            loot: Rect::new(300 + x, 10 + y, 200, 200),
            button_down: false,
        }
    }

    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas, texture: &sdl2::render::Texture) {
        //line
        canvas.set_draw_color(Color::RGB(88, 97, 101));
        canvas.fill_rect(self.line).unwrap();

        //loot
        canvas
            .copy(texture, None, self.loot)
            .expect("Texture couldn't be loaded");

        //button & overline
        if self.button_down {
            canvas.set_draw_color(Color::RGB(215, 120, 192));
            canvas.fill_rect(self.button).unwrap();

            if self.overline.width() > self.line.width() {
                self.overline.set_width(0);
            } else {
                self.overline.set_width(self.overline.width() + 1);
            }
            canvas.set_draw_color(Color::RGB(255, 234, 139));
            canvas.fill_rect(self.overline).unwrap();
        } else {
            canvas.set_draw_color(Color::RGB(121, 215, 120));
            canvas.fill_rect(self.button).unwrap();
            self.overline.set_width(0);
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();

    let window = video_subsystem
        .window("Melvor Idle Budget", 2560, 1600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut panel_one = Panel::new(0, 0);
    let mut panel_two = Panel::new(600, 0);
    let mut panel_three = Panel::new(1200, 0);

    let mut panels = Vec::new();
    panels.push(&mut panel_one);
    panels.push(&mut panel_two);
    panels.push(&mut panel_three);

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("../pictures/bonfire.png")
        .expect("Failed to load PNG");

    'running: loop {
        canvas.set_draw_color(Color::RGB(66, 75, 79)); //background color
        canvas.clear();

        //Handling events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    for panel in &mut panels {
                        if panel.button.contains_point((x, y)) {
                            panel.button_down = !panel.button_down;
                        }
                    }
                }
                _ => {}
            }
        }

        //Rendering panels
        for panel in &mut panels {
            panel.render(&mut canvas, &texture);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
