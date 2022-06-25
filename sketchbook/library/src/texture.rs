use nannou::prelude::*;
use super::math::point_inside_triangle;
//--------------------------------------------------------
// STIPPLE

struct TriangleStipple {

}

impl TriangleStipple {

}

pub struct StippleRect {
    pub position: Point2,
    pub c1:Point2,
    pub c2:Point2,
}

impl StippleRect {

    pub fn left(&self,  draw:&Draw, count: i32, point_size:f32) {

        let rect_w = abs(self.c1.x - self.c2.x);
        //let rect_h = abs(self.c1.y - self.c2.y);

        for i in 0..count {
            let rrand = rect_w * random::<f32>() * random::<f32>();
            let y = map_range(i as f32, 0.0, count as f32, self.c1.y, self.c2.y);
            let mut x = 0.0;
            if self.c1.x < self.c2.x {
                x = self.c1.x + rrand;
            } else {
                x = self.c2.x + rrand;
            }

            draw
            .translate(pt3(self.position.x, self.position.y, 0.0))
            .ellipse()
            .x_y(x, y)
            .w_h(point_size, point_size)
            .color(BLACK);
        }
    }

    pub fn outline(&self,  draw:&Draw, opacity:f32, fill: bool) {
        let c1 = self.c1;
        let c2 = self.c2;
        
        let p1 = c1;
        let p2 = pt2(c2.x, c1.y);
        let p3 = c2;
        let p4 = pt2(c1.x, c2.y);

        let points = [p1, p2, p3, p4];

        let fill_color = rgba(1.0, 1.0, 1.0, opacity);
        let stroke_color = rgba(0.0, 0.0, 0.0, opacity);
        if fill {
            draw
            .translate(pt3(self.position.x, self.position.y, 0.0))
            .polygon()
            .stroke_weight(1.0)
            .color(fill_color)
            .stroke_color(stroke_color)
            .points(points)
            ;
        } else {
            draw
            .translate(pt3(self.position.x, self.position.y, 0.0))
            .polyline()
            .stroke_weight(1.0)
            .color(stroke_color)
            .points(points)
            ;
        }
        
    }
}