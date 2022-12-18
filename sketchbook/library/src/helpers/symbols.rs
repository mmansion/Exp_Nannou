// basic helper symbols

use nannou::prelude::*;

pub fn draw_flowfield_arrow(draw: &Draw, points: Vec<Point2>) {
    draw.arrow()
        .points(points[0], points[1])
        .head_length(10.0)
        .head_width(10.0)
        .weight(1.0)
        .color(BLACK)
        .stroke_weight(1.0);
}

// pub struct FlowFieldArrow {
//     pub points: Vec<Point2>,
// }

// impl FlowFieldArrow {

//     pub fn new() -> Self {
//         let points = Vec::new();
//         FlowFieldArrow {
//             points,
//         }
//     }

//     pub fn draw(&self, draw: &Draw) {
//         draw.arrow()
//             .points(self.points[0], self.points[1])
//             .head_length(10.0)
//             .head_width(10.0)
//             .weight(1.0)
//             .color(BLACK)
//             .stroke_weight(1.0);
//     }
// }