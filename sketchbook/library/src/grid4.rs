use nannou::prelude::*;

// use crate::helpers::symbols::draw_flowfield_arrow as draw_flowfield_arrow;

pub struct Grid4 {
    rows: usize,
    cols: usize,
    rotation: f32,

    inner_margin: i32,
    outer_margin: i32,

    pub width: i32,
    pub height: i32,
    pub points: Vec<Vec2>, //grid points, corners of row/col lines
    pub cells : Vec<Vec2>, //grid cells, inbetween row/col lines

    // TODO: pickup up here
    pub corner_points: Vec<Vec<Vec2>>, //multi-dim array of points
    pub cell_points: Vec<Vec<Vec2>>, //multi-dim array of points

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
}

impl Grid4 {

    pub fn new(rows: usize, cols: usize, width: i32, height: i32) -> Self {
        let width = width;
        let height = height;
        let rows = rows;
        let cols = cols;
        let rotation = 0.0;

        //offset for nannou coord sys
        let y_off = -height as f32 / 2.0;
        let x_off = -width as f32 / 2.0;

        let mut points = Vec::new();
        let mut cells  = Vec::new();
        let mut corner_points = Vec::new();
        let mut cell_points = Vec::new();

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

        //--------------------------------------------------------
        //populate points
        for row in 0..(rows + 1) as usize {
            let f_height = height as f32;
            let f_rows = rows as f32;
            let f_row = row as f32;
            let y = (f_height / f_rows * f_row) + y_off;

            corner_points.push(Vec::new());
            cell_points.push(Vec::new());

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

                }
            }
        }

        //--------------------------------------------------------
        let inner_margin = 0;
        let outer_margin = 0;

        //--------------------------------------------------------

        Grid4 {
            cols,
            rows,
            width,
            height,
            
            points,
            cells,
            corner_points,
            cell_points,

            rotation,

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

            inner_margin,
            outer_margin,
        }
    }

    pub fn set_outer_margin(&mut self, margin: i32) {
        self.outer_margin = margin;
    }

    pub fn set_inner_margin(&mut self, margin: i32) {
        self.inner_margin = margin;
    }

    pub fn set_rows(&mut self, rows: usize) {
        if self.rows != rows {
            //update only if change
            self.rows = rows;
            self.update_points();
        }
    }
    pub fn set_cols(&mut self, cols: usize) {
        if self.cols != cols {
            //update only if change
            self.cols = cols;
            self.update_points();
        }
    }

    pub fn rotation(&mut self, radials: f32) {
        self.rotation = radials;
    }

    fn update_points(&mut self) {
        ////clears vecs and remove items from memory
        // self.points.clear(); 
        // self.cells.clear(); 

        self.corner_points.clear();
        self.cell_points.clear();

        for row in 0..(self.rows + 1) {
            let f_height = self.height as f32;
            let f_rows = self.rows as f32;
            let f_row = row as f32;
            let y = (f_height / f_rows * f_row) + self.y_off;

            self.corner_points.push(Vec::new());
            self.cell_points.push(Vec::new());

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

                }
            }
        }
    }

    fn draw_arrows(&self, draw: &Draw) {
        for p in 0..self.cell_points.len() {
            draw.arrow()
                .start(self.cells[p])
                .end(self.cells[p]+vec2(20.0, 0.0))
                .head_length(10.0)
                .head_width(2.0)
                .weight(2.0)
                .color(BLACK)
                .stroke_weight(1.0);
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
        //draw row lines
        for r in 0..self.corner_points.len() {
            for c in 0..self.corner_points[r].len() {
                //row line points
                let row_start_pt = self.corner_points[r][c];
                let row_end_pt = pt2(self.corner_points[r][c].x + self.width as f32, self.corner_points[r][c].y);

                // col line points
                let col_start_pt = self.corner_points[r][c];
                let col_end_pt = pt2(self.corner_points[r][c].x, self.corner_points[r][c].y + self.height as f32);

                draw.line()
                    .stroke_weight(self.line_weight)
                    .color(self.line_color)
                    .points(row_start_pt, row_end_pt);

                draw.line()
                    .stroke_weight(self.line_weight)
                    .color(self.line_color)
                    .points(col_start_pt, col_end_pt);
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let draw = draw.rotate(self.rotation);

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
    }
}
