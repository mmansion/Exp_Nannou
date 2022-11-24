use nannou::prelude::*;
// use super::points::Point;

//linear equation of a line

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
    pub m: f32, //slow of line segment
    pub b: f32, //y-intercept of line segment

    pub A: Vec2, //start pt of line segment
    pub B: Vec2, //end pt of line
    pub C: Vec2, //vertex @ 90d corner of right-tri, formed by line segment

    pub M: Vec2, //midpoint of line segment

    pub rise: f32,
    pub run: f32,

    pub normal_p1: Vec2,
    pub normal_p2: Vec2,

    color: Rgba,
}

impl Line {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        let mut A = p1;
        let mut B = p2;
        let mut C = vec2(0.0, 0.0);

        let mut m = (B.y - A.y) / (B.x - A.x);
        let mut b = m * A.x - A.y;
        let mut M = vec2(0.0, 0.0);

        let mut rise = 0.0;
        let mut run = 0.0;

        let mut normal_p1 = vec2(0.0, 0.0);
        let mut normal_p2 = vec2(0.0, 0.0);

        let mut color = rgba(0.0, 0.0, 0.0, 1.0);

        // -----------------------------------------

        Line {
            m,
            b,
            A,
            B,
            C,
            M,
            rise,
            run,
            normal_p1,
            normal_p2,
            color
        }
    }

    //checks if point is on line
    pub fn point_on_line(&self, test_point: Vec2, threshold: f32) -> bool {
        let x = test_point.x;
        let y = test_point.y;

        // point-slope form:
        // y = mx + b
        // does x and y satisfy the equation
        // println!("{}",abs(y - self.m * x + self.b));
        let diff = abs(y - self.m * x + self.b);
        return diff < threshold;
    }

    pub fn point_above_line(
        &self,
        test_point: Vec2,
        point_offset_x: f32,
        point_offset_y: f32,
    ) -> bool {
        let x = test_point.x + point_offset_x;
        let y = test_point.y + point_offset_y;

        // point-slope form:
        // y = mx + b

        let diff = y - self.m * x + self.b;
        return diff > 0.0;
    }

    pub fn get_slope(&self, A: Vec2, B: Vec2) -> f32 {
        let m = (B.y - A.y) / (B.x - A.x);
        return m;
    }

    pub fn get_y_at_x(&self, x: f32) -> f32 {
        //Solving "Ax + By = C" for "y ="

        let C = self.A.y - self.A.x * self.m;

        let y = self.m * x + C;

        /*
        var m = CalculateSlope(a, b);

        // Vertical line (y-values are always the same)
        if (double.IsPositiveInfinity(m))
            return a.Y;

        var c = a.Y - a.X * m;

        return Convert.ToInt32(m * x + c);
            */

        return y;
    }

    pub fn get_yintercept(&self, p: Vec2, m: f32) -> f32 {
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

    pub fn get_midpoint(&self) -> Vec2 {
        return vec2((self.A.x + self.B.x) / 2.0, (self.A.y + self.B.y) / 2.0);
    }

    pub fn update(&mut self) {
        self.update_normal();
    }

    pub fn update_points(&mut self, A: Vec2, B: Vector2) {
        // update A point
        self.A = A;

        // update B point
        self.B = B;

        // update slope
        self.m = self.get_slope(self.A, self.B);

        // update yintercept
        self.b = self.get_yintercept(self.A, self.m);
    }

    pub fn update_normal(&mut self) {
        let dx1 = self.B.x - self.A.x;
        let dy1 = self.B.y - self.A.y;

        self.normal_p1 = vec2(-dy1, dx1);
        self.normal_p2 = vec2(dy1, -dx1);
    }

    pub fn color(&mut self, c:Rgba) {
        self.color = c;
    }

    pub fn draw(&self, draw: &Draw) {
        // Draw a line!
        draw.line()
            .weight(1.0)
            .caps_round()
            .color(self.color)
            .points(self.A, self.B);
    }
}
