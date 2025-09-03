use raylib::prelude::*;

pub struct Particle {
    pub rad: f32,
    pub pos: Vector2,
    pub vel: Vector2,
    pub col: Color,
}

impl Particle {
    pub fn new(rad: f32, pos: Vector2, vel: Vector2, col: Color) -> Particle {
        Particle {
            rad,
            pos,
            vel,
            col,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.pos.x as i32, self.pos.y as i32, self.rad, self.col);
    }

    pub fn step(&mut self) {
        self.pos += self.vel;
    }

    // assume mass area density of 1
    pub fn mass(&self) -> f32 {
        self.rad * self.rad * PI as f32
    }
}
