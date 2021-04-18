use nannou::prelude::*;

pub struct Bezier {
    
    pub start_point     : Point2,
    pub end_point       : Point2,
    pub control_point_1 : Point2,
    pub control_point_2 : Point2,
    pub position        : Vector2,
    pub show_handles    : bool,

    stroke_weight : f32,
}

impl Bezier {

    pub fn new(
        start_point: Point2, 
        control_point1: Point2, 
        control_point_2: Point2, 
        end_point: Point2) -> Self {
        
        let start_point     = start_point;
        let end_point       = end_point;
        let control_point_1 = control_point1;
        let control_point_2 = control_point_2;

        let position = vec2(start_point.x, start_point.y);

        let show_handles = false;
        let stroke_weight = 1.0;

        //--------------------------------------------------------

        Bezier {
            start_point,
            end_point,
            control_point_1,
            control_point_2,
            position,
            show_handles,
            stroke_weight
        }
    }

    pub fn stroke_weight(&mut self, stroke_weight: f32) {
        self.stroke_weight = stroke_weight;
    }

    pub fn draw(&self, draw: &Draw) {

        let builder = nannou::geom::path::Builder::new();

        let path = builder
            .move_to( self.position )
            .cubic_bezier_to(
                self.control_point_1, 
                self.control_point_2, 
                self.end_point)
            .build();

        // draw the bezier curve path
        draw.path()
            .stroke()
            .weight(self.stroke_weight)
            .rgba(0.0, 0.0, 0.0, 1.0)
            .events(path.iter());

        if self.show_handles {
            // draw line from start to control point 1

            draw.line()
            .weight(1.0)
            .caps_round()
            .color(GRAY)
            .points(self.start_point, self.control_point_1);

            // draw line from start to control point 2

            draw.line()
                .weight(1.0)
                .caps_round()
                .color(GRAY)
                .points(self.control_point_2, self.end_point);

            // draw starting point
            draw.ellipse()
                .xy(self.start_point)
                .radius(5.0)
                .color(RED);

            // draw control point 1
            draw.ellipse()
                .xy(self.control_point_1)
                .radius(5.0)
                .color(BLUE);

            // draw control point 2
            draw.ellipse()
                .xy(self.control_point_2)
                .radius(5.0)
                .color(BLUE);

            // draw end point
            draw.ellipse()
                .xy(self.end_point)
                .radius(5.0)
                .color(RED);
        }
        
    }
}