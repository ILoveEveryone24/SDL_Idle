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
    health_rect: Rect,
    health: u32,
    line: Rect,
    overline: Rect,
    loot: Rect,
    button_down: bool,
}

impl Panel {
    fn new(x: i32, y: i32, hp: u32) -> Panel {
        Panel {
            button: Rect::new(300 + x, 360 + y, 200, 100),
            health_rect: Rect::new(300 + x, 220 + y, 50, 50),
            health: hp,
            line: Rect::new(150 + x, 280 + y, 500, 50),
            overline: Rect::new(150 + x, 280 + y, 0, 50),
            loot: Rect::new(300 + x, 10 + y, 200, 200),
            button_down: false,
        }
    }

    fn render(
        &mut self,
        canvas: &mut sdl2::render::WindowCanvas,
        texture: &sdl2::render::Texture,
        text: &sdl2::render::Texture,
        font: &sdl2::ttf::Font,
    ) {
        let texture_creator = canvas.texture_creator();
        //line
        canvas.set_draw_color(Color::RGB(88, 97, 101));
        canvas.fill_rect(self.line).unwrap();

        //loot
        canvas
            .copy(texture, None, self.loot)
            .expect("Texture couldn't be loaded");

        //hp
        canvas.set_draw_color(Color::RGB(255, 178, 102));
        canvas.fill_rect(self.health_rect).unwrap();

        //button & overline
        if self.button_down {
            canvas.set_draw_color(Color::RGB(215, 120, 192));
            canvas.fill_rect(self.button).unwrap();

            if self.overline.width() > self.line.width() {
                self.overline.set_width(0);
                if self.health > 0 {
                    self.health -= 1;
                }
            } else {
                self.overline.set_width(self.overline.width() + 10);
            }
            canvas.set_draw_color(Color::RGB(255, 234, 139));
            canvas.fill_rect(self.overline).unwrap();
        } else {
            canvas.set_draw_color(Color::RGB(121, 215, 120));
            canvas.fill_rect(self.button).unwrap();
            self.overline.set_width(0);
        }

        let hp_surface = font
            .render(&self.health.to_string())
            .blended(Color::RGBA(255, 255, 255, 255))
            .expect("No hp_surface");
        let hp = texture_creator
            .create_texture_from_surface(&hp_surface)
            .expect("Texture creator failed");

        //text on button
        canvas
            .copy(text, None, self.button)
            .expect("Texture couldn't be loaded");
        canvas
            .copy(&hp, None, self.health_rect)
            .expect("Texture couldn't be loaded");
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("Melvor Idle Budget", 2560, 1600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut panel_one = Panel::new(0, 0, 30);
    let mut panel_two = Panel::new(600, 0, 50);
    let mut panel_three = Panel::new(1200, 0, 10);

    let mut panels = Vec::new();
    panels.push(&mut panel_one);
    panels.push(&mut panel_two);
    panels.push(&mut panel_three);

    let texture_creator = canvas.texture_creator();

    let font = ttf_context
        .load_font("../fonts/OpenSans.ttf", 128)
        .expect("Couldn't load font");

    let surface = font
        .render("Press!")
        .blended(Color::RGBA(0, 0, 0, 255))
        .expect("No surface");
    let text = texture_creator
        .create_texture_from_surface(&surface)
        .expect("Texture creator failed");

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
            panel.render(&mut canvas, &texture, &text, &font);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
