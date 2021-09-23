use nannou::prelude::*;
use std::collections::VecDeque;
use super::line::Line;

pub struct Particle2 {

    pub history  : VecDeque<Vec2>,

    pub orig : Vec2,  // starting coord
    pub pos  : Vec2,  // screen coordinate
    pub vel  : Vec2,  // change in position over time
    pub acc  : Vec2,  // change in velocity over time

    //extras
    pub mass  : f32,
    pub speed : f32,
    pub size  : f32,

    pub max_speed : f32,
    pub last_pos : Vec2,
}

impl Particle2 {

    pub fn new(_x: f32, _y: f32, _mass:f32, _size:f32) -> Self {
        
        let history  = VecDeque::<Vec2>::with_capacity(1000);
        
        let mut orig     = vec2(_x, _y);
        let mut pos      = vec2(_x, _y);
        let mut size     = _size;
        let mut mass     = _mass;
        let mut last_pos = vec2(_x, _y);

        let vel = vec2(0.0, 0.0);
        let acc = vec2(0.0, 0.0);

        let max_speed = 50.0;
        let speed = 0.0;

        Particle2 {
            orig,
            pos,
            vel,
            acc,
            mass,
            speed,
            size,
            history,
            last_pos,
            max_speed,
        }
    }

    pub fn apply_force(&mut self, _force: Vec2) {
        let force = _force / self.mass; //Accel = Force/Mass
        self.acc += force;
    }

    pub fn avoid(&mut self, _target: Vec2) {
        let d = self.pos.distance(_target);
        
        let mut diff = vec2(0.0, 0.0);

        if d < self.size {
            diff = _target - self.pos;

            diff *= -1.0;

            diff = diff.normalize();
            // diff = diff.clamp_length_max(self.max_speed); //can use another speed for avoid

            diff *= self.max_speed;


        }

        // reynold's steering: steering = desired - velocity
        let mut steer = diff - self.vel;

        // steer = steer
        let max_avoid_force = 0.1;
        steer = steer.clamp_length_max(steer.length() * max_avoid_force);

        self.apply_force(steer);

        


    }

    pub fn update(&mut self) {
    
        // 1. add acceleration to velocity
        self.vel += self.acc;
        
        // 2. don't go over the speed limit! you'll get a ticket
        self.vel = self.vel.clamp_length_max(self.max_speed);

        // 3. preserve our last pos
        self.last_pos = self.pos;

        //4. update the position based on the velocity
        self.pos += self.vel;

        //Get the Euclidean distance between current and previous poss
        //let dist = self.pos.distance(self.last_pos);

        // println!("{}", dist);

        // 5. reset accelerate ea cycle
        self.acc *= 0.0; //reset
    }

    //particle collision with a line
    pub fn collide_line(&mut self, line:&Line) {
        

        // 1. adjust position (don't fall through line)
        self.pos.y = line.get_y_at_x(self.pos.x) + (self.size/2.0);//offset
        
        // 2. bounce

        // self.vel *= -self.vel * 0.1;
        self.vel += line.normal_p1;
        let dist = self.pos.distance(self.last_pos);
        self.vel = self.vel.clamp_length_max(dist);
        self.vel = self.vel.clamp_length_max(self.max_speed);

        // let mut diff =  line.normal_p1 - self.pos;
        let mut p = vec2(0.0, 0.0);
        if line.A.y < line.B.y {
            p = line.A;
        } else {
            p = line.B;
        }
        // let mut diff =  p - self.pos;

        // //diff *= -1.0;

        // diff = diff.normalize();
        //     // diff = diff.clamp_length_max(self.max_speed); //can use another speed for avoid

        // diff *= self.max_speed;

        // // reynold's steering: steering = desired - velocity
        // let mut steer = diff - self.vel;

        // // steer = steer
        // let max_avoid_force = 0.1;
        // steer = steer.clamp_length_max(steer.length() * max_avoid_force);

        // self.apply_force(steer);
        //let dist = self.pos.distance(self.last_pos);
        
        // self.vel = self.vel.clamp_length_max(self.max_speed);
    }

    pub fn display(&self, draw: &Draw) {
        // Display circle at x pos
        draw.ellipse()
            .xy(self.pos)
            .w_h(self.size, self.size)
            .rgba(0.0, 0.0, 0.0, 0.1)
            .stroke(WHITE)
            .stroke_weight(3.0);
    }

    pub fn display_line(&self, draw: &Draw) {
        let points = [
            self.orig,
            self.pos
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
        
    //     // let has_intersect = intersects_line(self.orig, self.pos, p1, p2);
    //     // if has_intersect {
    //     //     self.vel.y *= -1.0;
    //     //     // self.pos.y -= self.size;
    //     // }
    // }

    pub fn check_line_bounds(&mut self, line:&Line) {
        
        // if we fell below line
        if !line.point_above_line(self.pos, 0.0, 0.0) { 

            // if we're in range of the line's segment
            if self.pos.x > line.A.x && self.pos.x < line.B.x {

                self.pos.y = line.get_y_at_x(self.pos.x) + 0.0;
                self.vel.y *= -2.0;//diminish for friction of bounce
                
                //self.vel = line.A;

                self.apply_force(line.A);

                // self.vel.normalize();

                // TODO: https://stackoverflow.com/questions/61272597/calculate-the-bouncing-angle-for-a-ball-point
                /*
                As for how to get the surface normal
                if (x,y) is the vector from P1 to P2, then (-y,x) is perpendicular to it
                */

                //1. Get the surface normal
                // let p1 = line.A;
                // let p2 = line.B;
               // let surface_normal = see Processing Sketch / surface_normal


                //https://docs.rs/nannou/0.14.1/nannou/geom/vector/struct.Vec2.html#method.dot

                // if(self.pos.x < pt_used_for_angle.x) {
                //     // let rotate_x = self.vel.rotate(pt_used_for_angle.y.atan2(pt_used_for_angle.x)).x;
                //     // let rotate_y = self.vel.rotate(pt_used_for_angle.y.atan2(pt_used_for_angle.x)).y;
    
                //     // self.vel.x = rotate_x;
                //     // self.vel.y = rotate_y;
    
                //     // print!("{} ,", rotate_x);
                //     // println!("{}", rotate_y);
                // } else {
                //     // let rotate_x = self.vel.rotate(pt_used_for_angle.y.atan2(pt_used_for_angle.x) * PI).x;
                //     // let rotate_y = self.vel.rotate(pt_used_for_angle.y.atan2(pt_used_for_angle.x) * PI).y;
    
                //     // self.vel.x = rotate_x;
                //     // self.vel.y = rotate_y;
    
                //     // print!("{} ,", rotate_x);
                //     // println!("{}", rotate_y);
                // }

            }
        } 
        
        // println!("Checking line bounds");
    }

    // pub fn check_bounds(&mut self, rect: Rect) {
    //     if self.pos.x > rect.right() {
    //         self.pos.x = rect.right();
    //         self.vel.x *= -1.0;
    //     } else if self.pos.x < rect.left() {
    //         self.vel.x *= -1.0;
    //         self.pos.x = rect.left();
    //     }
    //     if self.pos.y < rect.bottom() {
    //         self.vel.y *= -1.0;
    //         self.pos.y = rect.bottom();
    //     }
    // }

    pub fn check_bounds(&mut self, rect: Rect) {
        
        let MARGIN = 0.0;

        let off_x = self.size/2.0;
        let off_y = self.size/2.0;

        if self.pos.y > rect.w()/2.0 - off_x  { //past top edge
            self.pos.y = rect.w()/2.0 - (self.size/2.0) - MARGIN;
            self.vel.y *= -1.0;
        } else 

        if self.pos.y < -rect.w()/2.0 + off_x { // past bottom edge
            self.pos.y = -rect.w()/2.0 + (self.size/2.0) + MARGIN;
            self.vel.y *= -1.0;
        } else 
    
        if self.pos.x < -rect.w()/2.0 + off_y { //past left edge
            self.pos.x = -rect.w()/2.0 + (self.size/2.0) + MARGIN;
            self.vel.x *= -1.0;
            
        } else 

        if self.pos.x > rect.w()/2.0 - off_y{ //past right edge
            self.pos.x = rect.w()/2.0 - (self.size/2.0) - MARGIN;
            self.vel.x *= -1.0;
        }
        self.vel = self.vel.clamp_length_max(self.max_speed*0.5);
    }
   

    // pub fn hasCollision(&mut self, point:Vec2, size:f32) -> bool {
    //     let v = self.pos - point; // Calculate direction of force
    //     let distance = v.magnitude(); // Distance between objects
    //     if distance <= size {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }

}