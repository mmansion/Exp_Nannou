use nannou::prelude::*;

// Next steps:
// why create a background library abstraction?

// - bg gradients
// - bg color transitions
// - bg transitions

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