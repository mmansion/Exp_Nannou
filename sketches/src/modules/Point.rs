mod Point;

// ---------------------------------------------------------------------------
struct Point {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    mass : f32,
    size : f32,
}
impl Point {
    fn new(x: f32, y: f32, m: f32, s: f32) -> Self {
        let mass = m;
        let position = pt2(x, y);
        let size = s;
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        Point {
            position,
            velocity,
            acceleration,
            mass,
            size,
        }
    }
    fn display(&self, draw: &Draw) {
        draw.ellipse()
        .xy(self.position)
        .radius( self.size )
        .color( GRAY);
    }
}