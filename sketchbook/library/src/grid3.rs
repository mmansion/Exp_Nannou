use nannou::prelude::*;

pub struct Grid3 {
    rows: i32,
    cols: i32,

    pub width     : i32,
    pub height    : i32,
    pub points    : Vec <Vec2>,

    x_off:f32,
    y_off:f32,
    
    pub point_size  : f32,
    pub line_weight : f32,
    pub line_color  : Rgba,
    pub point_color : Rgba,

    pub show_lines  : bool,
    pub show_points : bool
}

impl Grid3 {

    pub fn new(rows: i32, cols: i32, width: i32, height:i32) -> Self {
        
        let width    = width;
        let height   = height;

        let rows = rows;
        let cols = cols;

         //offset for coord sys
        let y_off = -height as f32 /2.0;
        let x_off = -width as f32 /2.0;

        let mut points    = Vec::new();

        //--------------------------------------------------------
        //default settings
        let point_size  = 5.0;
        let point_color = rgba(0.0, 0.0, 0.0, 1.0);
        let line_weight = 10.0;
        let line_color  = rgba(0.1, 0.1, 1.0, 1.0);
        
        let show_points = true;
        let show_lines  = true;

        //--------------------------------------------------------
        for row in 0..(rows+1) {
            for col in 0..(cols+1) {
                points.push(pt2(0.0, 0.0));
            } 
        }

        //--------------------------------------------------------

        Grid3 {
            cols,
            rows,
            width,
            height,
            points,

            y_off,
            x_off,
            
            point_size,
            point_color,
            line_weight,
            line_color,

            show_points,
            show_lines,
        }
    }

    pub fn rows(&mut self, rows: i32) {
        if self.rows != rows { //update only if change
            self.rows = rows;
            self.update_points();
        }

    }
    pub fn cols(&mut self, cols: i32) {
        if self.cols != cols { //update only if change
            self.cols = cols;
            self.update_points();
        }
    }

    fn update_points(&mut self) {

        self.points.clear(); //clears vec and removes items from memory

        for row in 0..(self.rows+1) {
            let f_height = self.height as f32;
            let f_rows = self.rows as f32;
            let f_row = row as f32;
            let y =  (f_height / f_rows * f_row) + self.y_off;
            
            for col in 0..(self.cols+1) {
                let f_width = self.width as f32;
                let f_cols = self.cols as f32;
                let f_col = col as f32;
                let x = (f_width / f_cols * f_col) + self.x_off;
                self.points.push(pt2(x, y));

            } 
        }
    }

    pub fn draw(&self, draw: &Draw) {


        //draw col lines

        for c in 0..(self.cols + 1) as usize {
            let start_pt = self.points[c];

            let end_pt = pt2(
                self.points[c].x, 
                self.points[c].y + self.height as f32 
            );

            draw
            .line()
            .stroke_weight(self.line_weight)
            .color(self.line_color)
            .points(start_pt, end_pt)
            ;
        }

         // draw grid lines 


        for r in 0..(self.rows + 1) as usize {
            let r = r * (self.cols + 1) as usize;
            let start_pt = self.points[r];
          
            if r == (self.points.len() - (self.cols + 1) as usize) {
                println!("{}", self.points[r]);

                draw.ellipse()
                .xy(self.points[r])
                .radius( 50.0 )
                .color(RED); 
            }

            let end_pt = pt2(
                self.points[r].x + self.width as f32, 
                self.points[r].y
            );

            draw
            .line()
            .stroke_weight(self.line_weight)
            .color(GREEN)
            .points(start_pt, end_pt)
            ;
        }

        // draw points

        for p in 0..self.points.len() {
            draw.ellipse()
            .xy(self.points[p])
            .radius( self.point_size )
            .color(BLACK); 
        }
        
    }
}