use nannou::prelude::*;
// use super::points::Point;

/*
public float   m; // slope of the line segment
public float   b; // y-intercept of the line
public PVector A; // startpoint of the line segment
public PVector B; // endpoint of the line segment
public PVector C; // vertex at 90 deg corner of the right-triangle formed by the segment //TODO: refactor
public PVector M; // Midpoint of the line segment
public float   l, length; // length of the line
*/

pub struct Line {

    pub m : f32, //slow of line segment
    pub b : f32, //y-intercept of line segment
    
    pub A : Vector2, //start pt of line segment
    pub B : Vector2, //end pt of line
    pub C : Vector2, //vertex @ 90d corner of right-tri, formed by line segment

    pub M : Vector2, //midpoint of line segment
}

impl Line {

    pub fn new(p1:Vector2, p2:Vector2) -> Self {
        
        let mut A = p1;
        let mut B = p2;
        let mut C = vec2(0.0, 0.0);

        let mut m = (B.y - A.y) / (B.x - A.x);
        let mut b = m * A.x - A.y;
        let mut M = vec2(0.0, 0.0);

        // -----------------------------------------

        Line {
            m, b, A, B, C, M
        }
    }

    pub fn point_on_line(&self, test_point:Vector2) -> bool {
        let x = test_point.x;
        let y = test_point.y;
        
        // point-slope form:
        // y = mx + b
        return y == self.m * x + self.b;
    }

    fn get_slope(&self, A: Vector2, B: Vector2) -> f32 {
        let m = (B.y - A.y) / (B.x - A.x);
        return m;
    }
    fn get_yintercept(&self, p:Vector2, m: f32) -> f32 {

        // EXAMPLE:
        // -p.y = m * p.x + b;
        // -p.y - b = m * p.x
        // -b = (m * p.x) + p.y;
        // (-1)(-b) = (-1)(m * p.x) + (-1)(p.y);
        // b = (-m * -p.x) - p.y;
        // b = (m * p.x) - p.y;

        let b = m * p.x - p.y;
        // b *= -1; //do we need to flip for axis?
        return b;
    }

    fn get_midpoint(&self) -> Vector2 {
        // todo
        return vec2(0.0, 0.0);
    }
    pub fn update(&self) {

    }

    pub fn update_points(&mut self, A:Vector2, B:Vector2) {

        // update A point
        self.A = A;
        
        // update B point
        self.B = B;

        // update slope
        self.m = self.get_slope(self.A, self.B);

        // update yintercept
        self.b = self.get_yintercept(self.A, self.m);
    }

    pub fn draw(&self, draw: &Draw) {
        
    }
}