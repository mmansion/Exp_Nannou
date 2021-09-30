use nannou::prelude::*;


struct ContLine {
    origin  : Vec2,
    history : Vec<Vec2>,
}

impl ContLine {

    fn new(origin:Vec2) -> Self {
        let origin = origin;
        let mut history = Vec::new();

        ContLine {
            origin,
            history,
        }
    }

    pub fn update(&mut self) {
        
    }

    pub fn display(&self) {
        
    }
}