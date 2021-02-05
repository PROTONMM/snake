use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;


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
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut iter = 0;
    let mut frame_x = 0;
    let mut frame_y = 0;

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
        canvas.copy(&texture, Rect::new(frame_x, frame_y, 26, 36), Rect::new(iter, 0, 26, 36))?;
        canvas.present();
        iter += 1;

        if iter >= 300 {
            iter = 0;
        }

        if iter%10==0 {
            frame_x += 26;
        }


        if frame_x >= 26*9 {
            frame_x = 0;
            frame_y += 36;
        }

        if frame_y >= 36*4 {
            frame_y = 0;
        }

        // Delay
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


