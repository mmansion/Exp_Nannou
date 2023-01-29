// Vechile (work in progress)

use nannou::prelude::*;
use std::collections::VecDeque;

use crate::helpers::nibs::draw_boid;

use super::points::Point;
use super::helpers::*;

pub struct Vehicle {
    pub history: VecDeque<Vec2>,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    // Maximum steering force
    pub max_force: f32,
    // Maximum speed
    max_speed: f32,
    mass: f32,
    margin: f32,
    line_len: usize,
    pub nib_size: f32,
    r: f32,
    pub nib_color: Rgb
}

impl Vehicle {
    pub fn new(x: f32, y: f32, max_speed: f32, v: Vec2, l: usize) -> Self {
        let mass = 10.0;
        let history = VecDeque::<Vec2>::with_capacity(1000);
        let line_len = l;
        let position = vec2(x, y);
        let mut velocity = v;
        let acceleration = vec2(0.0, 0.0);
        let max_force = 10.9;
        let max_speed = max_speed;
        let margin = 0.0;
        let r = 6.0;
        let nib_size = 2.0;
        let nib_color   = rgb(1.0, 1.0, 1.0);
        Vehicle {
            mass,
            history,
            position,
            velocity,
            acceleration,
            max_force,
            max_speed,
            margin,
            line_len,
            nib_size,
            nib_color,
            r,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_velocity(&self) -> Vec2 {
        // println!("velocity: {:?}", self.velocity);
        self.velocity.clone()
    }

    // Method to update position
    pub fn update(&mut self) {
        // Update velocity
        self.velocity += self.acceleration;
        // Limit speed
        self.velocity = self.velocity.clamp_length_max(self.max_speed);

        self.position += self.velocity;
        // Reset accelerationelertion to 0 each cycle
        self.acceleration *= 0.0;
        self.history.push_back(self.position);
        if self.history.len() > self.line_len {
            self.history.pop_front();
        }
    }


    pub fn display(&self, draw: &Draw) {
        // Draw a triangle rotated in the direction of velocity
        // This calculation is wrong
        let theta = (self.velocity.angle() + PI / 2.0) * -1.0;
        draw_boid(draw, self.position, self.nib_size, self.nib_color, theta);

        // let theta = (self.velocity.angle() + PI / 2.0) * -1.0;
        // let points = vec![
        //     pt2(0.0, -self.r * 2.0),
        //     pt2(-self.r, self.r * 2.0),
        //     pt2(self.r, self.r * 2.0),
        // ];
        // draw.polygon()
        //     .stroke(WHITE)
        //     .stroke_weight(1.0)
        //     .points(points)
        //     .xy(self.position)
        //     .rgb(0.5, 0.5, 0.5)
        //     .rotate(-theta);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        // We could add mass here if we want A = F / M
        self.acceleration += force;
    }

    // self must be mutable to update velocity
    pub fn redirect(&mut self, p: &Point) {
        let v = self.position - p.position; // Calculate direction of force
        let distance = v.length(); // Distance between objects

        let size = if p.size > 5.0 { p.size } else { 5.0 };

        if distance <= size {
            self.velocity = self.velocity.rotate(PI / (random_f32() * 4.0));
        }
    }

    pub fn redirect2(&mut self, point: Vec2, size: f32) {
        let v = self.position - point; // Calculate direction of force
        let distance = v.length(); // Distance between objects

        if distance <= size {
            self.velocity = self.velocity.rotate(PI / (random_f32() * 4.0));
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        self.velocity = self.velocity.rotate(angle);
    }

    pub fn hasCollision(&mut self, point: Vec2, size: f32) -> bool {
        let v = self.position - point; // Calculate direction of force
        let distance = v.length(); // Distance between objects
        if distance <= size {
            return true;
        } else {
            return false;
        }
    }

    pub fn boundaries_loop(&mut self, win: &Rect) {
        let left = win.left() + self.margin as f32;
        let right = win.right() - self.margin as f32;
        let top = win.top() - self.margin as f32;
        let bottom = win.bottom() + self.margin as f32;

        if self.position.x < left {
            self.position.x = right;
        } else if self.position.x > right {
            self.position.x = left;
        } else if self.position.y < bottom {
            self.position.y = top;
        } else if self.position.y > top {
            self.position.y = bottom;
        }
    }

    pub fn boundaries(&mut self, win: &Rect) {
        let left = win.left() + self.margin as f32;
        let right = win.right() - self.margin as f32;
        let top = win.top() - self.margin as f32;
        let bottom = win.bottom() + self.margin as f32;

        //update bc Vector2 was updated to geom::Vec2
        // https://docs.rs/nannou/0.18.0/nannou/glam/struct.Vec2.html

        let desired = if self.position.x < left {
            vec2(self.max_speed, self.velocity.y)
        } else if self.position.x > right {
            vec2(-self.max_speed, self.velocity.y)
        } else if self.position.y < bottom {
            vec2(self.velocity.x, self.max_speed)
        } else if self.position.y > top {
            vec2(self.velocity.x, -self.max_speed)
        } else {
            vec2(0.0, 0.0)
        };

        //if let Some(desired) = desired {
        let desired = desired.normalize() * self.max_speed;
        let steer = (desired - self.velocity).clamp_length_max(self.max_force);
        self.apply_force(steer);
        //}
    }

    pub fn boundaries_with_margin(&mut self, win: &Rect, margin: i32) {
        let left = win.left() + margin as f32;
        let right = win.right() - margin as f32;
        let top = win.top() - margin as f32;
        let bottom = win.bottom() + margin as f32;

        let desired = if self.position.x < left {
            vec2(self.max_speed, self.velocity.y)
        } else if self.position.x > right {
            vec2(-self.max_speed, self.velocity.y)
        } else if self.position.y < bottom {
            vec2(self.velocity.x, self.max_speed)
        } else if self.position.y > top {
            vec2(self.velocity.x, -self.max_speed)
        } else {
            self.velocity
        };
        //Returns self normalized to length 1.0 if possible, else returns zero.
        //In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be zero.
        let desired = desired.normalize_or_zero() * self.max_speed;

        let steer = (desired - self.velocity).clamp_length_max(self.max_force);
   
        self.apply_force(steer);
    }
}
