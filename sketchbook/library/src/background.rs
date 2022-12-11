use nannou::prelude::*;

// Next steps:
// why create a background library abstraction?

// - bg gradients
// - bg color transitions
// - bg transitions

pub struct Background {
    _w: f32,
    _h: f32,
    _gradient_start: Rgb, //how to make more gradient points
    _gradient_end: Rgb,
}

impl Background {

    pub fn new(w:f32, h:f32) -> Self {
        let _w = w;
        let _h = h;

        print!("{} {} ", _w, _h);
        
        Background {
            _w, _h, _gradient_start: rgb(0.0, 0.0, 0.0), _gradient_end: rgb(1.0, 1.0, 1.0)
        }
    }

    pub fn w(&self) -> f32 {
        self._w
    }
    pub fn h(&self) -> f32 {
        self._h
    }

    pub fn update(&mut self) {
        
    }

    pub fn set_gradient_colors(&mut self, start_col: Rgb, end_col: Rgb) {
        self._gradient_start = start_col;
        self._gradient_end = end_col;
    }

    pub fn draw_gradient( &self, draw: &Draw) {
        let _w = self._w;
        let _h = self._h;

        for i in 0.._w as i32 {
            let x = map_range(i, 0, _w as i32, -_w *0.5, _w *0.5);
            let s = vec3(self._gradient_start.red, self._gradient_start.green, self._gradient_start.blue);
            let e = vec3(self._gradient_end.red, self._gradient_end.green, self._gradient_end.blue);
            let gradient = s.lerp( e, map_range(x, -_w*0.5, _w*0.5, 0.0, 1.0) );
            
            draw.line()
                .start(pt2(x, -_h*0.5))
                .end(pt2(x, _h*0.5))
                .weight(1.0)
                .color(rgba(gradient.x, gradient.y, gradient.z, 1.0));
        }
      
    }

    pub fn draw(&self, draw: &Draw) {
    //   draw.background().color(self.color);
    }

}



pub struct BG_Color {
    pub color : Rgba,
}

impl BG_Color {
    pub fn new(c:Rgba) -> Self {
    
        let color = c;

        BG_Color {
            color
        }
    }

    pub fn update(&mut self) {
        
    }

    pub fn draw(&self, draw: &Draw) {
    
      draw.background().color(self.color);
    }

    
}