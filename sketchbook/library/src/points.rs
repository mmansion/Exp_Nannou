use nannou::prelude::*;

pub struct Point {
    pub position: Point2,
    pub velocity: Vector2,
    pub mass: f32,
    pub size: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, m: f32, s: f32) -> Self {
        let mass = m;
        let position = pt2(x, y);
        let size = s;
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        Point {
            position,
            velocity,
            mass,
            size,
        }
    }
    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.size)
            .color(GRAY);
    }
}
