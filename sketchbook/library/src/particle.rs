use nannou::prelude::*;
use std::collections::VecDeque;
use super::line::Line;
use super::math::intersects_line;

pub struct Particle {
    pub history  : VecDeque<Vector2>,
    pub origin   : Vector2,
    pub position : Vector2,
    pub velocity : Vector2,
    pub acceleration : Vector2,
    pub mass : f32,
    pub display_size : f32,
    pub max_speed : f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        let history  = VecDeque::<Vector2>::with_capacity(1000);
        
        let display_size = 10.0;
        let mut mass = 512.0;
        let position = vec2(x, y);
        let origin   = vec2(x, y);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let max_speed = 4.0;

        Particle {
            mass,
            display_size,
            history,
            position,
            origin,
            velocity,
            acceleration,
            max_speed
        }
    }

    // pub fn set_display_size(&mut self, value: f32) {
    //     self.siz
    // }

    // pub fn set_mass(&mut self, value: f32) {

    // }

    pub fn apply_force(&mut self, force: Vector2) {
        let f = force / (1000.0 - self.mass);
        self.acceleration += f;
    }

    pub fn update(&mut self) {
    
        // Update velocity
        self.velocity += self.acceleration;
        // Limit speed
        //self.velocity.clamp_length_max(self.max_speed);
        self.position += self.velocity;
        self.acceleration *= 0.0; //reset
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

    pub fn display_line(&self, draw: &Draw) {
        let points = [
            self.origin,
            self.position
            ];
        draw.scale(1.0)
            .polyline()
            .weight(2.0)
            .color(rgba(1.0, 1.0, 1.0, 1.0))
            .points(points)
            ;
    }

    
    // //deprecate this in lieu of line: point on line function
    // pub fn check_line_bounds(&mut self, p1:Point2, p2:Point2) {
        
    //     // let has_intersect = intersects_line(self.origin, self.position, p1, p2);
    //     // if has_intersect {
    //     //     self.velocity.y *= -1.0;
    //     //     // self.position.y -= self.display_size;
    //     // }
    // }

    pub fn check_line_bounds(&mut self, line:&Line) {
        

        if !line.point_above_line(self.position) { // if we fell below line

            if self.position.x > line.A.x && self.position.x < line.B.x {

                self.position.y = line.get_y_at_x(self.position.x) + 0.0;

            
                self.velocity.y *= -1.0;//friction of bounce
                

                // self.velocity.rotate(line.B.y.atan2(line.B.x)*-1.0);
             
                // let wind = vec2(0.01, 0.0);
                //self.apply_force(vec2(line.m * -5.0, 0.0));

                //acceleration

                //1. diff = here - there
                //2. target.sub(location)
                //3. set mag

                // A vector pointing from the position to the target
                // Scale to maximum speed
                let desired = (-self.position).normalize() * self.max_speed;

                self.acceleration = desired - self.velocity;

                // Steering = Desired minus velocity
                // Limit to maximum steering force
        //(desired - *velocity).clamp_length_max(*max_force)


                // let theta = (self.velocity.angle() + PI / 2.0) * -1.0;

                //self.velocity.rotate(self.velocity.angle() +  line.B.y.atan2(line.B.x));
               
                // self.velocity *= rad_to_deg( line.B.y.atan2(line.B.x) );
                

            }
        } 
        
        // println!("Checking line bounds");
    }

    pub fn check_edges(&mut self, rect: Rect) {
        // if self.position.x > rect.right() {
        //     self.position.x = rect.right();
        //     self.velocity.x *= -1.0;
        // } else if self.position.x < rect.left() {
        //     self.velocity.x *= -1.0;
        //     self.position.x = rect.left();
        // }
        // if self.position.y < rect.bottom() {
        //     self.velocity.y *= -1.0;
        //     self.position.y = rect.bottom();
        // }
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