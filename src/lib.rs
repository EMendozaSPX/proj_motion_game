extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CannonBall {
    gravity: f64,
    velocity: f64,
    x_velocity: f64,
    y_velocity: f64,
    time: f64,
    max_height_t: f64,
}

#[wasm_bindgen]
impl CanonBall {
    pub fn new(&initial_velocity: f64, &initial_angle: f64, &initial_time: i64) -> CannonBall {
        let gravity: f64 = -9.8;
        let velocity: f64 = initial_velocity;
        let x_velocity: f64 = init_x_vel(initial_velocity, initial_angle);
        let y_velocity: f64 = init_y_vel(initial_velocity, initial_angle);
        let time: f64 = initial_time as f64 / 1000.0;
        let max_height_t: f64 = max_height_t(y_velocity, gravity);
        CannonBall {
            gravity,
            velocity,
            x_velocity,
            y_velocity,
            time,
            max_height_t,
        }
    }

    pub fn update(&mut self, &deltaTime: i64) {
        let my: f64 = self.max_height;
        self.time += deltaTime as f64 / 1000.0;

        if self.time >= self.max_height_t {
            self.gravity = -self.gravity;
        }

        let t: f64 = self.time;
        let uy: f64 = self.y_velocity;

        let ay: f64 = self.gravity;
        let vy: f64 = vel(uy, ay, t);

        self.y_velocity = vy;
    }

    pub fn get_x_vel(&self) -> i64 {
        self.x_velocity.round()
    }

    pub fn get_y_vel(&self, &screen_height: i64) -> i64 {
        screen_height - self.y_velocity.round()
    }
}

fn init_x_vel(&velocity: f64, &angle: f64) -> f64 {
    velocity * angle.cos()
}

fn init_y_vel(&velocity: f64, &angle: f64) -> f64 {
    velocity * angle.sin()
}

fn vel(&u: f64, &a: f64, &t: f64) -> f64 {
    u + (a * t)
}

fn max_height_t(&u: f64, &a: f64) -> f64 {
    (-u) / a
}
