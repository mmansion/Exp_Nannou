mod Vehicle;

struct Vehicle {
    history: VecDeque<Vector2>,
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    r: f32,
    // Maximum steering force
    max_force: f32,
    // Maximum speed
    max_speed: f32,
    mass : f32,
}

impl Vehicle {
    fn new(x: f32, y: f32, v: Vector2) -> Self {
        let mass     = 10.0;
        let history  = VecDeque::<Vector2>::with_capacity(100);
        let position = vec2(x, y);
        let velocity = v;
        let acceleration = vec2(0.0, 0.0);
        let r = 100.0;
        let max_force = 1.09;
        let max_speed = 1.0;

        Vehicle {
            mass,
            history,
            position,
            velocity,
            acceleration,
            r,
            max_force,
            max_speed,
        }
    }

    // Method to update position
    fn update(&mut self) {
        // Update velocity
        self.velocity += self.acceleration;
        // Limit speed
        self.velocity.limit_magnitude(self.max_speed);
        self.position += self.velocity;
        // Reset accelerationelertion to 0 each cycle
        self.acceleration *= 0.0;
        self.history.push_back(self.position);
        if self.history.len() > LINE_LEN {
            self.history.pop_front();
        }
    }

    fn apply_force(&mut self, force: Vector2) {
        // We could add mass here if we want A = F / M
        self.acceleration += force;
    }

    // Draw a triangle rotated in the direction of velocity
    // This calculation is wrong
    // let theta = (velocity.angle() + PI / 2.0) * -1.0;
    // let points = vec![pt2(0.0, -r * 2.0), pt2(-r, r * 2.0), pt2(*r, r * 2.0)];
    // draw.polygon()
    //     .stroke(BLACK)
    //     .stroke_weight(1.0)
    //     .points(points)
    //     .xy(*position)
    //     .rgb(0.5, 0.5, 0.5)
    //     .rotate(-theta);

    fn repel(&self, p: &Point) -> Vector2 {

        let mut force = self.position - p.position; // Calculate direction of force
        // let mut force = self.position; // Calculate direction of force
        let mut distance = force.magnitude(); // Distance between objects

        
        if(distance < 10.0) {
            //println!("{}", distance);
            self.velocity.rotate(PI/2.0);
        }

        distance = distance.max(1.0).min(10000.0); // Limiting the distance to eliminate "extreme" results for very cose or very far object
        force = force.normalize(); // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let g = 100.0;
        let strength = (g * self.mass * p.mass) / (distance * distance); // Calculate gravitational force magnitude
        force * (-1.0 * strength) // Get force vector --> magnitude * direction
       // force * (-1.0)
        // force;
        
    }

    fn boundaries(&mut self, d: f32, win: &Rect) {
        
        let left = win.left() + MARGIN as f32;
        let right = win.right() - MARGIN as f32;
        let top = win.top() - MARGIN as f32;
        let bottom = win.bottom() + MARGIN as f32;

        let desired = match self.position {
            Vector2 { x, .. } if x < left => Some(vec2(self.max_speed, self.velocity.y)),
            Vector2 { x, .. } if x > right => Some(vec2(-self.max_speed, self.velocity.y)),
            Vector2 { y, .. } if y < bottom => Some(vec2(self.velocity.x, self.max_speed)),
            Vector2 { y, .. } if y > top => Some(vec2(self.velocity.x, -self.max_speed)),
            _ => None,
        };

        if let Some(desired) = desired {
            let desired = desired.normalize() * self.max_speed;
            let steer = (desired - self.velocity).limit_magnitude(self.max_force);
            self.apply_force(steer);
        }
    }
}