use nannou::prelude::*;
use super::points::Point;

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
        
        let A = p1;
        let B = p2;
        let C = vec2(0.0, 0.0);

        let m = 0.0;
        let b = 0.0;
        let M = vec2(0.0, 0.0);

        // SETUP -----------------------------------

        // -----------------------------------------

        Line {
            m, b, A, B, C, M
        }
    }

    pub fn update(&self) {

    }

    pub fn draw(&self, draw: &Draw) {
        
    }
}