use nannou::prelude::*;

pub struct ContinousLine {
    pub history: Vec<Vec2>,
    pub stroke: Rgba,
    pub weight: f32,
    pub origin: Vec2,
}

impl ContinousLine {
    pub fn new(origin: Vec2) -> Self {
        let origin = origin;
        let mut history = Vec::new();

        //default 1.0pt black line
        let stroke = rgba(0.0, 0.0, 0.0, 1.0);
        let weight = 1.0;

        history.push(origin);

        ContinousLine {
            origin,
            history,
            stroke,
            weight,
        }
    }

    pub fn set_stroke(&mut self, rgba: Rgba) {
        self.stroke = rgba;
    }

    pub fn set_weight(&mut self, w: f32) {
        self.weight = w;
    }

    pub fn display(&self, draw: &Draw) {
        let vertices = self.history.iter().map(|v| pt2(v.x, v.y));

        draw.polyline()
            .stroke_weight(self.weight)
            .caps_round()
            .color(self.stroke)
            .points(vertices);
    }
}
