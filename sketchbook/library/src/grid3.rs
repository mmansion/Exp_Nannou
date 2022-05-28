use nannou::prelude::*;

pub struct Grid3 {
    rows: i32,
    cols: i32,

    pub width     : i32,
    pub height    : i32,
    pub points    : Vec <Vec2>,
    pub angles    : Vec<Vec2>,

    row_start_pts : Vec <Vec2>,
    col_start_pts : Vec <Vec2>,

    x_off:i32,
    y_off:i32,
    
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
        let y_off = -height/2;
        let x_off = -width/2;

        let mut points    = Vec::new();
        let mut angles    = Vec::new();
        let mut row_start_pts = Vec::new();
        let mut col_start_pts = Vec::new();

        //--------------------------------------------------------
        //default settings
        let point_size  = 5.0;
        let point_color = rgba(0.0, 0.0, 0.0, 1.0);
        let line_weight = 1.0;
        let line_color  = rgba(0.1, 0.1, 0.1, 1.0);
        

        let show_points = true;
        let show_lines  = true;

        //--------------------------------------------------------
        for row in 0..(rows+1) {
            let y =  (height / rows * row + y_off) as f32;
            for col in 0..(cols+1) {
                let x =  (width / cols  * col + x_off) as f32;
                points.push(pt2(x, y));
                angles.push(vec2(1.0, 1.0));

                if row == 0 {
                    col_start_pts.push(pt2(x, y));
                }
                if col == 0 {
                    row_start_pts.push(pt2(x,y));
                }
            } 
        }

        //--------------------------------------------------------

        Grid3 {
            cols,
            rows,
            width,
            height,
            points,
            row_start_pts,
            col_start_pts,
            angles,

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
            println!("updating rows");
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
        self.points.clear(); //clears vec and remove from memory
        self.row_start_pts.clear();
        self.col_start_pts.clear();
        self.angles.clear();

        // self.points = Vec::new();

        for row in 0..(self.rows+1) {

            let y =  (self.height / self.rows * row + self.y_off) as f32;
            
            for col in 0..(self.cols+1) {

                let x =  (self.width / self.cols  * col + self.x_off) as f32;
                
                self.points.push(pt2(x, y));

                self.angles.push(vec2(1.0, 1.0));

                if row == 0 {
                    self.col_start_pts.push(pt2(x, y));
                }
                if col == 0 {
                    self.row_start_pts.push(pt2(x,y));
                }
            } 
        }
    }

    pub fn set_angle(&mut self, index:usize, angle:Vec2) {
        self.angles[index] = angle;
    }

    pub fn draw(&self, draw: &Draw) {

        // draw row lines
        for r in 0..self.row_start_pts.len() {
   
            let start_pt = self.row_start_pts[r];

            let end_pt = pt2(
                self.row_start_pts[r].x + self.width as f32, 
                self.row_start_pts[r].y );

            draw
            .line()
            .stroke_weight(self.line_weight)
            .color(self.line_color)
            .points(start_pt, end_pt)
            ;
        }

        // draw col lines
        for c in 0..self.col_start_pts.len() {

            let start_pt = self.col_start_pts[c];

            let end_pt = pt2(
                self.col_start_pts[c].x, 
                self.col_start_pts[c].y + self.height as f32 );

            draw
            .line()
            .stroke_weight(self.line_weight)
            .color(self.line_color)
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