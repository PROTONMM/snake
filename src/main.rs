use sdl2::pixels::{Color, PixelFormat};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use std::time::Duration;

pub struct Tail {
    pub x_pos: i32,
    pub y_pos: i32,
}

struct Snake {
    pub x_pos: i32,
    pub y_pos: i32,
    pub angle: i32,
    pub tail: Vec<Tail>,
}

impl Snake {
    pub fn new(x:i32, y:i32, angle:i32) -> Snake {
        Snake {x_pos: x, y_pos: y, angle: angle, tail: Vec::new() }
    }
    pub fn add_tail(&mut self, x:i32, y:i32) {
        let t = Tail {x_pos: x, y_pos: y};
        self.tail.push(t);
    }
}


fn main() -> Result<(), String>{

    let sdl = sdl2::init()?;

    let video_subsystem = sdl.video()?;
    let window = video_subsystem
        .window("Game", 800, 600)
        // .opengl()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let snake_head = texture_creator.load_texture("resources/snake_h.png")?;
    let snake_body = texture_creator.load_texture("resources/snake_b.png")?;

    let mut snnake = Snake::new(100,100,90);


    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];



    let mut event_pump = sdl.event_pump()?;
    'main: loop {
        //Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Right), ..
                } => println!("Right"),
                Event::KeyDown {
                    keycode: Some(Keycode::Left), ..
                } => println!("Left"),
                _ => {},
            }
        }
        // Render

        canvas.clear();

        // canvas.copy(&texture,None, None)?;
        canvas.copy(&snake_head, Rect::new(0, 0, 40, 40), Rect::new(0, 0, 40, 40))?;
        canvas.present();


        // Delay
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


