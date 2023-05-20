use nannou::prelude::*;
// use observer::{};

// use crate::helpers::symbols::draw_flowfield_arrow as draw_flowfield_arrow;

pub enum RECT_MODE { //todo: implement
    CORNER,
    CENTER,
}

pub struct Grid5 {
    pub grid_pos: Vec2,

    pub rows: usize,
    pub cols: usize,

    rotation: f32,
    scale  : f32,

    pub rect_mode: RECT_MODE,

    inner_margin: i32,
    outer_margin: i32,

    orig_w: f32,
    orig_h: f32,
    pub width: f32,
    pub height: f32,

    // pub points: Vec<Vec2>, //grid points, corners of row/col lines
    // pub cells : Vec<Vec2>, //grid cells, inbetween row/col lines

    // TODO: pickup up here
    pub corner_points: Vec<Vec<Vec2>>, //multi-dim array of points
    pub cell_points: Vec<Vec<Vec2>>, 
    
    pub cell_angles: Vec<Vec<f32>>, 
    cell_angles_editable: Vec<Vec<bool>>,
    // pub cell_angle: f32, //set by editor

    x_off: f32,
    y_off: f32,

    //visualizing grid
    pub corner_point_size: f32,
    pub cell_point_size: f32,

    pub corner_point_color: Rgba,
    pub cell_point_color: Rgba,

    pub line_weight: f32,
    pub line_color: Rgba,

    pub show_lines: bool,
    pub show_corner_points: bool,
    pub show_cell_points: bool,
    pub show_arrows: bool, //for flowfield
    pub show_cells: bool,

    pub edit_mode: bool,
    pub is_reset: bool,

    // pub on_resize: Box<fn()>,
    pub on_resize: fn(grid: &mut Grid5),

}

impl Grid5 {

    pub fn new(pos:Vec2, width: f32, height: f32, rows: usize, cols: usize) -> Self {

        let rect_mode: RECT_MODE = RECT_MODE::CORNER;
        let grid_pos = pos;

        // let margin = 100.0;

        //maintain orig width/height for scaling
        let orig_w = width;
        let orig_h = height;
        
        let width  = width;
        let height = height;
        let rows = rows;
        let cols = cols;
        let rotation = 0.0;
        let scale = 1.0;

        //offset for nannou's center origin coord sys

        //positioning for CORNER mode
        // let y_off = -height as f32 / 2.0;
        let y_off = grid_pos.y - height as f32;
        
        // let x_off = -width as f32 / 2.0;
        let x_off = grid_pos.x;

        // let mut points = Vec::new();
        // let mut cells  = Vec::new();
        let mut corner_points = Vec::new();
        let mut cell_points = Vec::new();

        let mut cell_angles = Vec::new();
        let mut cell_angles_editable = Vec::new();

        //--------------------------------------------------------
        //default settings
        let corner_point_size = 5.0;
        let cell_point_size = 2.0;

        let corner_point_color = rgba(0.0, 0.0, 0.0, 1.0);
        let cell_point_color   = rgba(0.0, 0.0, 1.0, 1.0);
    
        let line_weight = 1.0;
        let line_color = rgba(0.1, 0.1, 0.1, 1.0);

        let show_corner_points = false; //default
        let show_cell_points = true; //default
        let show_lines = false;
        let show_arrows = false;
        let show_cells = true;

        let edit_mode = false;
        let is_reset = false;
        //--------------------------------------------------------

        //populate points
        for row in 0..(rows + 1) as usize {
            let f_height = height as f32;
            let f_rows = rows as f32;
            let f_row = row as f32;
            let y = (f_height / f_rows * f_row) + y_off;

            corner_points.push(Vec::new());
            cell_points.push(Vec::new());
            cell_angles.push(Vec::new());
            cell_angles_editable.push(Vec::new());

            for col in 0..(cols + 1) as usize {
                let f_width = width as f32;
                let f_cols = cols as f32;
                let f_col = col as f32; 
                let x = (f_width / f_cols * f_col) + x_off;
                // points.push(pt2(x, y));
                corner_points[row].push(pt2(x, y));

                //calculate cell position
                if row < rows && col < cols {
                    let cell_x = (f_width / f_cols * f_col) + x_off + (f_width / f_cols / 2.0);
                    let cell_y = (f_height / f_rows * f_row) + y_off + (f_height / f_rows / 2.0);
                    //cells.push(pt2(cell_x, cell_y));

                    cell_points[row].push(pt2(cell_x, cell_y));
                    cell_angles[row].push(0.0);
                    cell_angles_editable[row].push(false);//default

                }
            }
        }

        //--------------------------------------------------------
        let inner_margin = 0;
        let outer_margin = 0;

        //--------------------------------------------------------

        Grid5 {
            grid_pos,
            cols,
            rows,
            orig_w,
            orig_h,
            width,
            height,

            rect_mode,
            
            // points,
            // cells,
            corner_points,
            cell_points,
            cell_angles,
            cell_angles_editable,

            rotation,
            scale,

            y_off,
            x_off,

            corner_point_size,
            cell_point_size,

            corner_point_color,
            cell_point_color,

            line_weight,
            line_color,

            show_cell_points,
            show_corner_points,
            show_lines,
            show_arrows,
            show_cells,
            edit_mode,
            is_reset,

            inner_margin,
            outer_margin,
          
            on_resize: |grid| {},
        }
    }


    pub fn get_num_rows(&self) -> usize {
        self.rows
    }

    pub fn get_num_cols(&self) -> usize {
        self.cols
    }

    pub fn get_nearest_cell_angle(&self, pos: Vec2) -> f32 {
        let mut min_dist = 999999.0;
        let mut angle = 0.0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let dist = pos.distance(self.cell_points[row][col]);
                if dist < min_dist {
                    min_dist = dist;
                    angle = self.cell_angles[row][col];
                }
            }
        }
        angle
    }

    pub fn get_nearest_cell_pos(&self, pos: Vec2) -> Vec2 {
        let mut min_dist = 999999.0;
        let mut cell_pos = vec2(0.0, 0.0);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let dist = pos.distance(self.cell_points[row][col]);
                if dist < min_dist {
                    min_dist = dist;
                    cell_pos = self.cell_points[row][col];
                }
            }
        }
        cell_pos
    }

    fn get_nearest_index(&self, pos: Vec2) -> (usize, usize) {
        let mut min_dist = 999999.0;
        let mut index = (0, 0);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let dist = pos.distance(self.cell_points[row][col]);
                if dist < min_dist {
                    min_dist = dist;
                    index = (row, col);
                }
            }
        }
        index
    }

    pub fn toggle_editable_cell(&mut self, pos: Vec2) {
        let index = self.get_nearest_index(pos);
        let editable = !self.cell_angles_editable[index.0][index.1];
        self.cell_angles_editable[index.0][index.1] = editable;
    }

    pub fn set_editable_cells_angle(&mut self, a:f32) {
        for row in 0..self.cell_points.len() {
            for col in 0..self.cell_points[row].len() {
                if self.cell_angles_editable[row][col] {
                    self.cell_angles[row][col] = a;
                }
            }
        }
    }

     // yes, you can pass a function to a function expecting a closure, 
    // no, you cannot pass closure to a function expecting a function 
    // https://stackoverflow.com/questions/52696907/why-does-passing-a-closure-to-function-which-accepts-a-function-pointer-not-work
    pub fn set_angles(&mut self, f:  impl Fn(Vec2, usize, usize)-> f32) {
        for row in 0..self.cell_points.len() {
            for col in 0..self.cell_points[row].len() {
                let angle = f( vec2(row as f32, col as f32), self.rows, self.cols);
                self.cell_angles[row][col] = angle;
            }
        }
    }
    
   
    pub fn set_angles_by_index(&mut self, f:  impl Fn(Vec2, usize, usize)-> f32) {
        for row in 0..self.cell_points.len() {
            for col in 0..self.cell_points[row].len() {
                let angle = f( vec2(row as f32, col as f32), self.rows, self.cols);
                self.cell_angles[row][col] = angle;
            }
        }
    }
    
    pub fn set_line_color(&mut self, color: Rgba) {
        self.line_color = color;
    }

    pub fn set_outer_margin(&mut self, margin: i32) {
        self.outer_margin = margin;
    }

    pub fn set_inner_margin(&mut self, margin: i32) {
        self.inner_margin = margin;
    }

    pub fn set_rows_cols(&mut self, rows: usize, cols: usize) {
        if self.rows != rows || self.cols != cols {
            //update only if change
            self.rows = rows;
            self.cols = cols;

            //resize only if change
            self.resize_grid();
        }
    }

    pub fn set_rows(&mut self, rows: usize) {
        if self.rows != rows {
            //update only if change
            self.rows = rows;
            self.resize_grid();
        }
    }
    pub fn set_cols(&mut self, cols: usize) {
        if self.cols != cols {
            //update only if change
            self.cols = cols;
            self.resize_grid();
        }
    }
    pub fn set_rotation(&mut self, radials: f32) {
        self.rotation = radials;
    }
    pub fn set_scale(&mut self, scale: f32) {
        // self.scale = scale;
        self.width  = self.orig_w * scale;
        self.height = self.orig_h * scale;
        self.resize_grid();
    }

    pub fn get_cell(&mut self, x:usize, y:usize) -> Vec2 {
        self.cell_points[y][x]
    }

    fn resize_grid(&mut self) {
        
        self.corner_points.clear();
        self.cell_points.clear();

        self.cell_angles.clear();
        self.cell_angles_editable.clear();

        for row in 0..(self.rows + 1) {
            let f_height = self.height as f32;
            let f_rows = self.rows as f32;
            let f_row = row as f32;
            let y = (f_height / f_rows * f_row) + self.y_off;

            self.corner_points.push(Vec::new());
            self.cell_points.push(Vec::new());
            self.cell_angles.push(Vec::new());
            self.cell_angles_editable.push(Vec::new());

            for col in 0..(self.cols + 1) {
                let f_width = self.width as f32;
                let f_cols = self.cols as f32;
                let f_col = col as f32;
                let x = (f_width / f_cols * f_col) + self.x_off;
                
                // self.points.push(pt2(x, y));
                self.corner_points[row].push(pt2(x, y));

                //calculate cell position
                if row < self.rows && col < self.cols {
                    let cell_x = (f_width / f_cols * f_col) + self.x_off + (f_width / f_cols / 2.0);
                    let cell_y = (f_height / f_rows * f_row) + self.y_off + (f_height / f_rows / 2.0);
                    // self.cells.push(pt2(cell_x, cell_y));

                    self.cell_points[row].push(pt2(cell_x, cell_y));
                    self.cell_angles[row].push(0.0); //no angle
                    self.cell_angles_editable[row].push(false); //not editable
                }
            }
        }
        (self.on_resize)(self);//call resize callback
   
    }

    fn draw_arrows(&self, draw: &Draw) {
        for row in 0..self.cell_points.len() {
            for col in 0..self.cell_points[row].len() {
                let x = self.cell_points[row][col].x;
                let y = self.cell_points[row][col].y;

                let draw = draw.translate(vec3(x,y,0.0));

                draw.arrow()
                    .rotate(self.cell_angles[row][col])
                    .start(vec2(-10.0, 0.0))
                    .end(vec2(10.0, 0.0))
                    .head_length(10.0)
                    .head_width(2.0)
                    .weight(2.0)
                    .color(self.cell_point_color)
                    .stroke_weight(1.0);
            }
        }
    }

    fn draw_corner_points(&self, draw: &Draw) {
        for row in 0..self.corner_points.len() {
            for col in 0..self.corner_points[row].len() {
                draw.ellipse()
                    .xy(self.corner_points[row][col])
                    .radius(self.corner_point_size)
                    .color(self.corner_point_color);
            }
        }
    }

    fn draw_cell_points(&self, draw: &Draw) {
        //draw cell points
        for row in 0..self.cell_points.len() {
            for col in 0..self.cell_points[row].len() {
                draw.ellipse()
                    .xy(self.cell_points[row][col])
                    .radius(self.cell_point_size)
                    .color(self.cell_point_color);
            }
        }
    }

    fn draw_grid_lines(&self, draw: &Draw) {      
        //draw column lines
        for c in 0..self.corner_points[0].len() {
            // col line points
            let col_start_pt = self.corner_points[0][c];
            let col_end_pt = pt2(self.corner_points[0][c].x, self.corner_points[0][c].y + self.height as f32);

            draw.line()
                .stroke_weight(self.line_weight)
                .color(self.line_color)
                .points(col_start_pt, col_end_pt);
        }
        //draw row lines
        for r in 0..self.corner_points.len() {
            // row line points
            let row_start_pt = self.corner_points[r][0];
            let row_end_pt = pt2(self.corner_points[r][0].x + self.width as f32, self.corner_points[r][0].y);

            draw.line()
                .stroke_weight(self.line_weight)
                .color(self.line_color)
                .points(row_start_pt, row_end_pt);
        }
    }

    pub fn draw(&self, draw: &Draw) {
        // let draw = draw.scale(self.scale).rotate(self.rotation);
        // let draw = draw.rotate(self.rotation);

        // draw grid lines
        if self.show_lines {
            self.draw_grid_lines(&draw);
        }
        
        // draw points
        if self.show_cell_points {
            self.draw_cell_points(&draw);
        }

        if self.show_corner_points {
            self.draw_corner_points(&draw);
        }

        // draw flow field arrows
        if self.show_arrows {
            self.draw_arrows(&draw);
        }

        // draw editable cell highlights
        self.draw_cell_highlights(&draw);

    }

    pub fn draw_cell_highlights(&self, draw: &Draw) {
        
        for i in 0..self.cell_angles_editable.len() {
            for j in 0..self.cell_angles_editable[i].len() {
                if self.cell_angles_editable[i][j] {
                    let x = self.cell_points[i][j].x;
                    let y = self.cell_points[i][j].y;
                    let w = self.width as f32 / self.cols as f32;
                    let h = self.height as f32 / self.rows as f32;

                    let draw = draw.translate(vec3(x,y,0.0));

            
                    draw.polyline()
                        .color(BLACK)
                        .weight(3.0)
                        .points(
                            vec![
                                pt2(-w/2.0, -h/2.0),
                                pt2(w/2.0, -h/2.0),
                                pt2(w/2.0, h/2.0),
                                pt2(-w/2.0, h/2.0),
                                pt2(-w/2.0, -h/2.0),
                            ]
                        );
                }
            }
        }

    }

}
