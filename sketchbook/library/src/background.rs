use nannou::prelude::*;

// Next steps:
// why create a background library abstraction?

// - bg gradients
// - bg color transitions
// - bg transitions

pub struct Background {
    w: f32,
    h: f32,
    gradient_start: Rgb, //how to make more gradient points
    gradient_end: Rgb,
    colors: Vec<Rgb>,
    gradiant_pts: Vec<f32>,
}

impl Background {

    pub fn new(w:f32, h:f32) -> Self {
        let w = w;
        let h = h;
        let colors = Vec::new();
        let gradiant_pts = Vec::new();

        print!("{} {} ", w, h);
        
        Background {
            colors,
            w, h, 
            gradiant_pts,
            gradient_start: rgb(0.0, 0.0, 0.0), gradient_end: rgb(1.0, 1.0, 1.0)
        }
    }
    pub fn set_colors(&mut self, colors: Vec<Rgb>) {
        
        self.colors = colors;
        
        let n_colors = self.colors.len() as f32;
        let frac = 1.0 / n_colors;
        let x_off = self.w * frac * 0.5 * -1.0;
        
        println!("");

        for i in 1..self.colors.len() + 1 {
            
            println!("{} ", self.w * frac * i as f32 + x_off);
            self.gradiant_pts.push( self.w * frac * i as f32 + x_off);
        }

    }

    pub fn w(&self) -> f32 {
        self.w
    }
    pub fn h(&self) -> f32 {
        self.h
    }

    pub fn update(&mut self) {
        
    }

    pub fn set_gradientcolors(&mut self, start_col: Rgb, end_col: Rgb) {
        self.gradient_start = start_col;
        self.gradient_end = end_col;
    }

    pub fn draw( &self, draw: &Draw) {
        let w = self.w;
        let h = self.h;

        let draw = draw.translate( vec3( -(w * 0.5), 0.0, 0.0) );


        for i in 0..w as i32 {

            // let x = map_range(i, 0, w as i32, -w *0.5, w *0.5);
            let x = i as f32;

            let lerp_zone_width = w / self.colors.len() as f32;
            
            let color_index = (i as f32 / lerp_zone_width).floor() as usize;

            // let mut c1:Rgb = rgb(0.0, 0.0, 0.0);
            // let mut c2:Rgb = rgb(0.0, 0.0, 0.0);
            // let is_last_zone = color_index == self.colors.len() - 1;

            // if x < self.gradiant_pts[self.gradiant_pts.len() - 1] {
                

            //     if x > self.gradiant_pts[color_index] {

            //     }
            // }

            
         

            if  
                x < self.gradiant_pts[self.gradiant_pts.len()-1] &&
                x > self.gradiant_pts[color_index] &&
                x < (self.gradiant_pts[color_index] + lerp_zone_width * 0.5) {
            
                    let c1 = self.colors[color_index];
                    let c2 = self.colors[color_index+1];
                    let col_start = vec3(c1.red, c1.green, c1.blue);
                    let col_end   = vec3(c2.red, c2.green, c2.blue); 
                    let gradient = col_start.lerp( col_end, map_range(x, self.gradiant_pts[color_index], self.gradiant_pts[color_index+1], 0.0, 1.0) );
    
                    draw.line()
                        .start(pt2(x, -h*0.5))
                        .end(pt2(x, h*0.5))
                        .weight(1.0)
                        .color(rgba(gradient.x, gradient.y, gradient.z, 1.0));
            
                } else if 
                    x > self.gradiant_pts[0] &&
                    x < self.gradiant_pts[self.gradiant_pts.len()-1] {

                    let c1 = self.colors[color_index];
                    let c2 = self.colors[color_index-1];
                    let col_start = vec3(c1.red, c1.green, c1.blue);
                    let col_end   = vec3(c2.red, c2.green, c2.blue); 
                    let gradient = col_start.lerp( col_end, map_range(x, self.gradiant_pts[color_index], self.gradiant_pts[color_index-1], 0.0, 1.0) );

    
                    draw.line()
                        .start(pt2(x, -h*0.5))
                        .end(pt2(x, h*0.5))
                        .weight(1.0)
                        .color(rgba(gradient.x, gradient.y, gradient.z, 1.0));

                }  else {
                    draw.line()
                        .start(pt2(x, -h*0.5))
                        .end(pt2(x, h*0.5))
                        .weight(1.0)
                        .color(self.colors[color_index]);
                    }

        
        }
      
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