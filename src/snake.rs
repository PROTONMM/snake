use crate::static_values;

pub struct Tail {
    pub x_pos: f64,
    pub y_pos: f64,
}

pub struct Snake {
    default_x_pos: f64,
    default_y_pos: f64,
    default_angle: f64,
    default_tail_len: i32,
    pub x_pos: f64,
    pub y_pos: f64,
    pub angle: f64,
    pub step: i32,
    pub tail: Vec<Tail>,
}

impl Snake {
    pub fn new(x:f64, y:f64, def_angle:f64, def_tail_len: i32) -> Snake {
        Snake {
            default_x_pos: x,
            default_y_pos: y,
            default_angle: def_angle,
            default_tail_len: def_tail_len,
            x_pos: x,
            y_pos: y,
            angle: def_angle,
            step: 0,
            tail: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        self.x_pos = self.default_x_pos;
        self.y_pos = self.default_y_pos;
        self.angle = self.default_angle;
        self.tail.clear();
        self.add_tail(self.default_tail_len);
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
        if self.step % static_values::STEP_TAIL == 0 {
            for i in (1..self.tail.len()).rev() {
                self.tail[i].y_pos = self.tail[i - 1].y_pos;
                self.tail[i].x_pos = self.tail[i - 1].x_pos;
            }
            self.tail[0].x_pos = self.x_pos;
            self.tail[0].y_pos = self.y_pos;
        }
        let pi_angle = self.angle /180.0 * std::f64::consts::PI;
        let step_x = pi_angle.cos()*static_values::STEP_LEN;
        let step_y = pi_angle.sin()*static_values::STEP_LEN;
        self.y_pos += step_y;
        self.x_pos += step_x;
    }

    pub fn collision_detection(&self) -> bool {
        let start_len = static_values::TAIL_START_LEN as usize;
        for i in start_len..self.tail.len() {
            let distance = ((self.x_pos - self.tail[i].x_pos).powf(2.0) +
                (self.y_pos - self.tail[i].y_pos).powf(2.0)).sqrt();

            if distance < static_values::TAIL_COLLISION_DISTANCE {
                println!("i: {}, snake_len {}", i, self.tail.len());
                return true;
            }
        }
        false
    }
}