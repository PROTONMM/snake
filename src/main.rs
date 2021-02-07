// use sdl2::pixels::{Color, PixelFormat};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
// use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::LoadTexture;
// use sdl2::pixels::PixelFormatEnum;
// use sdl2::surface::Surface;
use rand::Rng;
use std::time::Duration;


static STEP_LEN: f64 = 1.0;
static STEP_ANGLE: f64 = 3.0;
static STEP_TAIL: i32 = 3;
static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

pub struct Apple {
    pub x_pos: f64,
    pub y_pos: f64,
}

fn random_place() -> (f64,f64) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(40..SCREEN_WIDTH-40) as f64;
    let y = rng.gen_range(40..SCREEN_HEIGHT-40) as f64;
    (x,y)
}

impl Apple {
    pub fn new() -> Apple {
        let (x,y) = random_place();
        Apple {x_pos: x, y_pos: y }
    }

    pub fn update(&mut self) {
        let (x,y) = random_place();
        self.x_pos = x;
        self.y_pos = y;
    }
}

pub struct Tail {
    pub x_pos: f64,
    pub y_pos: f64,
}

struct Snake {
    pub x_pos: f64,
    pub y_pos: f64,
    pub angle: f64,
    pub step: i32,
    pub tail: Vec<Tail>,
}

impl Snake {
    pub fn new(x:f64, y:f64, angle:f64) -> Snake {
        Snake {x_pos: x, y_pos: y, angle: angle, step: 0, tail: Vec::new() }
    }
    pub fn add_tail(&mut self, count: i32) {
        let x;
        let y;
        if self.tail.len() == 0 {
            x = self.x_pos;
            y = self.y_pos;
        }else {
            x = self.tail[self.tail.len()-1].x_pos;
            y = self.tail[self.tail.len()-1].y_pos;
        }
        for _ in 0..count {
            let t = Tail { x_pos: x, y_pos: y };
            self.tail.push(t);
        }
    }

    pub fn next_step(&mut self) {
        // increment step
        self.step += 1;
        // step modulo
        if self.step % STEP_TAIL == 0 {
            for i in (1..self.tail.len()).rev() {
                self.tail[i].y_pos = self.tail[i - 1].y_pos;
                self.tail[i].x_pos = self.tail[i - 1].x_pos;
            }
            self.tail[0].x_pos = self.x_pos;
            self.tail[0].y_pos = self.y_pos;
        }
        let pi_angle = self.angle /180.0 * std::f64::consts::PI;
        let step_x = pi_angle.cos()*STEP_LEN;
        let step_y = pi_angle.sin()*STEP_LEN;
        self.y_pos += step_y;
        self.x_pos += step_x;
    }
}



fn main() -> Result<(), String>{

    let sdl = sdl2::init()?;

    let video_subsystem = sdl.video()?;
    let window = video_subsystem
        .window("Game", SCREEN_WIDTH, SCREEN_HEIGHT)
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
    let snake_tail = texture_creator.load_texture("resources/snake_t.png")?;
    let apple_image = texture_creator.load_texture("resources/apple1.png")?;
    let mut snake = Snake::new(100.0,100.0,90.0);

    let mut apple = Apple::new();

    snake.add_tail(30);
    let mut event_pump = sdl.event_pump()?;

    //***********************************************************
    // MAIN LOOP
    //***********************************************************
    'main: loop {
        //***********************************************************
        //Events
        //***********************************************************
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'main,
                _ => {},
            }
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Left) {
            snake.angle -= STEP_ANGLE;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Right) {
            snake.angle += STEP_ANGLE;
        }

        //***********************************************************
        //Collisions
        //***********************************************************
        //eat apple
        let apple_head_distance = ((snake.x_pos - apple.x_pos).powf(2.0) + (snake.y_pos - apple.y_pos).powf(2.0)).sqrt();
        if apple_head_distance < 20.0 {
            apple.update();
            snake.add_tail(10);
        }

        //wall collision
        if snake.x_pos + 20.0 < 0.0 {
            snake.x_pos += SCREEN_WIDTH as f64;
        }
        if snake.x_pos + 20.0 > SCREEN_WIDTH as f64 {
            snake.x_pos -= SCREEN_WIDTH as f64;
        }
        if snake.y_pos + 20.0 < 0.0 {
            snake.y_pos += SCREEN_HEIGHT as f64;
        }
        if snake.y_pos + 20.0 > SCREEN_HEIGHT as f64 {
            snake.y_pos -= SCREEN_HEIGHT as f64;
        }


        //***********************************************************
        //Calculate
        //***********************************************************
        //calculate tail angle.
        let mut x_pos = snake.tail[snake.tail.len() - 1].x_pos - snake.tail[snake.tail.len() - 2].x_pos;
        let y_pos = snake.tail[snake.tail.len() - 1].y_pos - snake.tail[snake.tail.len() - 2].y_pos;
        if x_pos == 0.0 {
            x_pos = 0.00001;
        }

        let mut tail_angle = (y_pos / x_pos).atan() * 180.0 / std::f64::consts::PI;
        if x_pos < 0.0 {
            tail_angle = tail_angle + 180.0;
        }

        if x_pos > 0.0 && y_pos < 0.0 {
            tail_angle = tail_angle + 360.0;
        }

        //***********************************************************
        // Render
        //***********************************************************
        canvas.clear();
        //render tail
        canvas.copy_ex(&snake_tail,
                       Rect::new(0, 0, 40, 40),
                       Rect::new((snake.tail[snake.tail.len() - 1].x_pos) as i32,
                                 (snake.tail[snake.tail.len() - 1].y_pos) as i32,
                                 40,
                                 40),
                       tail_angle - 90.0,
                       Point::new(20, 20),
                       false, false)?;

        // render body
        for i in (1..snake.tail.len() - 1).rev() {
            if i % 10 == 0 {
                // let x_pos = (-snake.tail[i + 1].x_pos + snake.tail[i].x_pos) / STEP_TAIL as f64 * (snake.step % STEP_TAIL) as f64;
                // let y_pos = (-snake.tail[i + 1].y_pos + snake.tail[i].y_pos) / STEP_TAIL as f64 * (snake.step % STEP_TAIL) as f64;
                canvas.copy_ex(&snake_body,
                               Rect::new(0, 0, 40, 40),
                               Rect::new((snake.tail[i].x_pos) as i32,
                                         (snake.tail[i].y_pos) as i32,
                                         40,
                                         40),
                               90.0,
                               Point::new(20, 20),
                               false, false)?;
            }
    }
        //render head
        canvas.copy_ex(&snake_head,
                       Rect::new(0, 0, 40, 40),
                       Rect::new(snake.x_pos as i32, snake.y_pos as i32, 40, 40),
                       snake.angle - 90.0,
                       Point::new(20,20),
                       false, false)?;


        //render apple
        canvas.copy_ex(&apple_image,
                       Rect::new(0, 0, 40, 40),
                       Rect::new(apple.x_pos as i32, apple.y_pos as i32, 40, 40),
                       0.0,
                       Point::new(20,20),
                       false, false)?;

        canvas.present();
        snake.next_step();

        // Delay
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


