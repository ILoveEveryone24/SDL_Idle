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
    button_outline: Rect,
    health_rect: Rect,
    health_rect_outline: Rect,
    line: Rect,
    overline: Rect,
    line_outline: Rect,
    loot: Rect,
    damage: u32,
    button_down: bool,
    dead: bool,
}

impl Panel {
    fn new(x: i32, y: i32, dmg: u32) -> Panel {
        Panel {
            button: Rect::new(300 + x, 360 + y, 200, 100),
            button_outline: Rect::new(295 + x, 355 + y, 210, 110),
            health_rect: Rect::new(200 + x, 225 + y, 400, 40),
            health_rect_outline: Rect::new(195 + x, 220 + y, 410, 50),
            line: Rect::new(150 + x, 280 + y, 500, 50),
            line_outline: Rect::new(145 + x, 275 + y, 510, 60),
            overline: Rect::new(150 + x, 280 + y, 0, 50),
            loot: Rect::new(300 + x, 10 + y, 200, 200),
            damage: dmg,
            button_down: false,
            dead: false,
        }
    }

    fn render(
        &mut self,
        canvas: &mut sdl2::render::WindowCanvas,
        texture: &sdl2::render::Texture,
        button_text: &sdl2::render::Texture,
    ) {
        //line
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        match canvas.fill_rect(self.line_outline) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("An error ocurred: {:?}", error);
            }
        }
        canvas.set_draw_color(Color::RGB(88, 97, 101));
        match canvas.fill_rect(self.line) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("An error ocurred: {:?}", error);
            }
        }

        //loot
        match canvas.copy(texture, None, self.loot) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("An error ocurred: {:?}", error);
            }
        }

        //hp
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        match canvas.fill_rect(self.health_rect_outline) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("An error ocurred: {:?}", error);
            }
        }
        if self.health_rect.width() > 1 {
            canvas.set_draw_color(Color::RGB(210, 0, 0));
            match canvas.fill_rect(self.health_rect) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("An error ocurred: {:?}", error);
                }
            }
        } else {
            self.dead = true;
        }

        //button & overline
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        match canvas.fill_rect(self.button_outline) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("An error ocurred: {:?}", error);
            }
        }
        if self.button_down && !self.dead {
            canvas.set_draw_color(Color::RGB(215, 120, 192));
            match canvas.fill_rect(self.button) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("An error ocurred: {:?}", error);
                }
            }

            if self.overline.width() > self.line.width() {
                self.overline.set_width(0);
                if self.health_rect.width() > self.damage {
                    self.health_rect
                        .set_width(self.health_rect.width() - self.damage);
                } else {
                    self.health_rect.set_width(0);
                }
            } else {
                self.overline.set_width(self.overline.width() + 500);
            }
            canvas.set_draw_color(Color::RGB(255, 234, 139));
            match canvas.fill_rect(self.overline) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("An error ocurred: {:?}", error);
                }
            }
        } else if self.dead {
            canvas.set_draw_color(Color::RGB(66, 75, 79));
            match canvas.fill_rect(self.button) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("An error ocurred: {:?}", error);
                }
            }
        } else {
            canvas.set_draw_color(Color::RGB(121, 215, 120));
            match canvas.fill_rect(self.button) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("An error ocurred: {:?}", error);
                }
            }
            self.overline.set_width(0);
        }

        //button_text on button
        match canvas.copy(button_text, None, self.button) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("An error ocurred {:?}", error);
            }
        }
    }
}

pub fn main() {
    let sdl_context = match sdl2::init() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't initialize sdl context: {:?}", error),
    };
    let video_subsystem = match sdl_context.video() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't initialize video_subsystem: {:?}", error),
    };
    let _image_context = match sdl2::image::init(InitFlag::PNG) {
        Ok(x) => x,
        Err(error) => panic!("Couldn't initialize image context: {:?}", error),
    };
    let ttf_context = match sdl2::ttf::init() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't initialize ttf context: {:?}", error),
    };

    let window = match video_subsystem
        .window("Melvor Idle Budget", 2560, 1600)
        .position_centered()
        .resizable()
        .build()
    {
        Ok(x) => x,
        Err(error) => panic!("Couldn't create a window: {:?}", error),
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't create canvas: {:?}", error),
    };

    let mut event_pump = match sdl_context.event_pump() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't catch events: {:?}", error),
    };

    let mut panels = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut dmg = 41;
    for _ in 0..8 {
        if width <= 1800 {
            panels.push(Panel::new(width, height, dmg));
            width += 600;
        }
        if height < 500 && width > 1800 {
            width = 0;
            height = 500;
        }
        dmg -= 5;
    }

    let texture_creator = canvas.texture_creator();

    let textures = vec![
        texture_creator
            .load_texture("../pictures/bonfire.png")
            .expect("Failed to load bonfire"),
        texture_creator
            .load_texture("../pictures/spear.png")
            .expect("Failed to load spear"),
        texture_creator
            .load_texture("../pictures/anvil.png")
            .expect("Failed to load anvil"),
        texture_creator
            .load_texture("../pictures/factory.png")
            .expect("Failed to load factory"),
        texture_creator
            .load_texture("../pictures/chemistry.png")
            .expect("Failed to load chemistry"),
        texture_creator
            .load_texture("../pictures/rocket.png")
            .expect("Failed to load rocket"),
        texture_creator
            .load_texture("../pictures/computer.png")
            .expect("Failed to load computer"),
        texture_creator
            .load_texture("../pictures/linux.png")
            .expect("Failed to load linux"),
    ];

    let font = ttf_context
        .load_font("../fonts/OpenSans.ttf", 256)
        .expect("Couldn't load font");

    let surface_button = font
        .render("Press!")
        .blended(Color::RGBA(0, 0, 0, 255))
        .expect("No surface_button");
    let button_text = texture_creator
        .create_texture_from_surface(&surface_button)
        .expect("Texture creator failed");

    let surface_win_text = font
        .render("You won!")
        .blended(Color::RGBA(0, 0, 0, 255))
        .expect("No surface_win_button_text");
    let win_text = texture_creator
        .create_texture_from_surface(&surface_win_text)
        .expect("Texture creator failed");

    let win_text_rect = Rect::new(780, 1100, 1000, 300);

    'running: loop {
        //Background color
        canvas.set_draw_color(Color::RGB(66, 75, 79));
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
                        if !panel.dead {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        //Rendering panel
        for (index, panel) in panels.iter_mut().enumerate() {
            let texture = &textures[index];
            panel.render(&mut canvas, texture, &button_text);
            if !panel.dead {
                break;
            }
        }
        let you_win = panels.iter().all(|panel| panel.dead);
        if you_win {
            canvas
                .copy(&win_text, None, win_text_rect)
                .expect("Texture couldn't be loaded");
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
