use nannou::prelude::*;
use std::collections::VecDeque;

// use super::points::Point;

pub struct Particle {
    pub history  : VecDeque<Vector2>,
    pub origin   : Vector2,
    pub position : Vector2,
    pub velocity : Vector2,
    pub acceleration : Vector2,
    pub mass : f32,
    pub display_size : f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        let history  = VecDeque::<Vector2>::with_capacity(1000);
        
        let display_size     = 10.0;
        let mass     = 10.0;
        let position = vec2(x, y);
        let origin   = vec2(x, y);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);

        Particle {
            mass,
            display_size,
            history,
            position,
            origin,
            velocity,
            acceleration,
        }
    }

    // pub fn set_display_size(&mut self, value: f32) {
    //     self.siz
    // }

    // pub fn set_mass(&mut self, value: f32) {

    // }

    pub fn apply_force(&mut self, force: Vector2) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    pub fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .xy(self.position)
            .w_h(self.display_size, self.display_size)
            .rgba(0.0, 0.0, 0.0, 0.1)
            .stroke(WHITE)
            .stroke_weight(2.0);
    }

    pub fn check_edges(&mut self, rect: Rect) {
        if self.position.x > rect.right() {
            self.position.x = rect.right();
            self.velocity.x *= -1.0;
        } else if self.position.x < rect.left() {
            self.velocity.x *= -1.0;
            self.position.x = rect.left();
        }
        if self.position.y < rect.bottom() {
            self.velocity.y *= -1.0;
            self.position.y = rect.bottom();
        }
    }
   

    // pub fn hasCollision(&mut self, point:Vector2, size:f32) -> bool {
    //     let v = self.position - point; // Calculate direction of force
    //     let distance = v.magnitude(); // Distance between objects
    //     if distance <= size {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }

}