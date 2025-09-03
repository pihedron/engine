use raylib::prelude::*;
mod particle;
use particle::Particle;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

fn round(v: Vector2) -> Vector2 {
    Vector2::new(v.x.round(), v.y.round())
}

fn simulate(obj: &mut Vec<Particle>) -> bool {
    let mut collided = false;

    for p in &mut *obj {
        p.step();

        if p.pos.x + p.rad > WIDTH as f32 {
            p.pos.x = WIDTH as f32 - p.rad;
            p.vel.x *= -1.0;
        }

        if p.pos.x - p.rad < 0.0 {
            p.pos.x = p.rad;
            p.vel.x *= -1.0;
        }

        if p.pos.y + p.rad > HEIGHT as f32 {
            p.pos.y = HEIGHT as f32 - p.rad;
            p.vel.y *= -1.0;
        }

        if p.pos.y - p.rad < 0.0 {
            p.pos.y = p.rad;
            p.vel.y *= -1.0;
        }
    }

    for i in 0..obj.len() {        
        for j in i..obj.len() {
            if i == j {
                continue;
            }

            let ds = obj[j].pos - obj[i].pos;
            let unit = ds.scale_by(1.0 / ds.length());
            let sum_rad = obj[i].rad + obj[j].rad;

            if ds.length() < sum_rad {
                let depth = (unit.scale_by(sum_rad) - ds).scale_by(0.5);

                obj[i].pos -= depth;
                obj[j].pos += depth;

                let sum_mass = obj[i].mass() + obj[j].mass();

                let dv = unit.scale_by((obj[j].vel - obj[i].vel).dot(unit));

                let dv_i = dv.scale_by(2.0 * obj[j].mass() / sum_mass);
                let dv_j = -dv.scale_by(2.0 * obj[i].mass() / sum_mass);

                obj[i].vel += round(dv_i);
                obj[j].vel += round(dv_j);

                collided = true;
            }
        }
    }

    collided
}

fn main() {
    let mut obj = vec![
        Particle::new(
            40.0,
            Vector2::new(80.0, HEIGHT as f32 / 2.0),
            Vector2::new(10.0, 0.0),
            Color::ROYALBLUE,
        ),
        Particle::new(
            20.0,
            Vector2::new(WIDTH as f32, HEIGHT as f32 / 2.0),
            Vector2::new(-20.0, 0.0),
            Color::SALMON,
        ),
        Particle::new(
            30.0,
            Vector2::new(1000.0, 200.0),
            Vector2::new(-20.0, 20.0),
            Color::PALEGOLDENROD,
        ),
    ];

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Pihedron Collider")
        .vsync()
        .build();

    let mut paused = false;
    let mut counter = 0;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused ^= true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            simulate(&mut obj);

            println!("\n[{counter}]");
            let mut energy = 0.0;
            for p in &obj {
                energy += p.mass() * p.vel.length() * p.vel.length() / 2.0;
                dbg!(p.vel);
            }
            println!("sum(E_k): {energy}");
            counter += 1;
        }

        if !paused {
            simulate(&mut obj);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for p in &mut obj {
            p.draw(&mut d);
        }
    }
}
