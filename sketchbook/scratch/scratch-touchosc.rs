use nannou::prelude::*;
use std::collections::HashMap;

//--------------------------------------------------------
fn main() {

    let mut touchosc = TouchOscClient::new();

    let fader1 = touchosc.add_fader("/fader1", 0.0, 10.0, 3.0);
    let fader2 = touchosc.add_fader("/fader2", 0.0, 1.0, 1.0);

    let button1 = touchosc.add_button("/button1", false);
    let button2 = touchosc.add_button("/button2", true);

    let grid = touchosc.add_grid("/grid", 2, 0.0, 100.0, 50.0);

    println!("fader value = {}", touchosc.fader("/fader1"));
    println!("button value = {}", touchosc.button("/button1"));

    println!("grid value 1 = {}", touchosc.grid("/grid", 1) );
}
//--------------------------------------------------------

#[derive(Debug)]
pub enum TouchOscInputType { Button, Fader, Grid, Encoder, Radar, Radial, Radio, XY }
pub struct TouchOscClient {
    //reference
    lookup_table : HashMap<String, TouchOscInputType>,

    //inputs
    touchosc_buttons  : HashMap<String, TouchOscButton>,
    touchosc_faders   : HashMap<String, TouchOscFader>,
    touchosc_grids    : HashMap<String, TouchOscGrid>,
    touchosc_encoders : HashMap<String, TouchOscEncoder>,
    touchosc_radars   : HashMap<String, TouchOscRadar>,
    touchosc_radials  : HashMap<String, TouchOscRadial>,
    touchosc_radios   : HashMap<String, TouchOscRadio>,
    touchosc_xys      : HashMap<String, TouchOscXY>,
}

impl TouchOscClient {
    pub fn new() -> Self {
        TouchOscClient {
            lookup_table      : HashMap::new(),
            touchosc_buttons  : HashMap::new(),
            touchosc_faders   : HashMap::new(),
            touchosc_grids    : HashMap::new(),
            touchosc_encoders : HashMap::new(),
            touchosc_radars   : HashMap::new(),
            touchosc_radials  : HashMap::new(),
            touchosc_radios   : HashMap::new(),
            touchosc_xys      : HashMap::new(),
        }
    }
    pub fn add_button(&mut self, addr:&str, default:bool) {
        self.verify_free_addr(addr);
        self.lookup_table.insert((&addr).to_string(),TouchOscInputType::Button);
        self.touchosc_buttons.insert((&addr).to_string(), TouchOscButton::new(default));
    }
    pub fn add_fader(&mut self, addr:&str, min:f32, max:f32, default:f32) {
        self.verify_free_addr(addr);
        self.lookup_table.insert((&addr).to_string(),TouchOscInputType::Fader);
        self.touchosc_faders.insert((&addr).to_string(), TouchOscFader::new(min, max, default));
    }
    pub fn add_grid(&mut self, addr:&str, size:usize, min:f32, max:f32, default:f32) {
        self.verify_free_addr(addr);
        self.lookup_table.insert((&addr).to_string(),TouchOscInputType::Grid);
        self.touchosc_grids.insert((&addr).to_string(), TouchOscGrid::new(size, min, max, default));
    }
    pub fn add_encoder() {

    }
    pub fn add_radar() {

    }
    pub fn add_radial() {

    }
    pub fn add_radio() {

    }
    pub fn add_xy(&mut self, path:&str, min:f32, max:f32, default:f32) {
       // self.touchosc_xys.insert(path.to_string(), TouchOscXY::new(path, min, max, default));
    }
    pub fn button(&self, addr:&str) -> bool {
        self.verify_has_addr(addr);
        for (key, input_type) in &self.lookup_table {
            if key == addr {
                return match &input_type { 
                    TouchOscInputType::Button => { self.touchosc_buttons[addr].state() },
                    _ => { false }
                };
            }
        } return false;
    }
    pub fn fader(&self, addr:&str) -> f32 {
        self.verify_has_addr(addr);
        for (key, input_type) in &self.lookup_table {
            if key == addr {
                return match &input_type { //verify correct type at addr
                    TouchOscInputType::Fader => { self.touchosc_faders[addr].value() },
                    _ => { 0.0 }
                };
            }
        } return 0.0;
        
    }
    pub fn grid(&self, addr:&str, index:usize) -> f32 {
        self.verify_has_addr(addr);
        for (key, input_type) in &self.lookup_table {
            if key == addr {
                return match &input_type { //verify correct type at addr
                    TouchOscInputType::Grid => { self.touchosc_grids[addr].value(index) },
                    _ => { 0.0 }
                };
            }
        } return 0.0;
        
    }
    pub fn verify_has_addr(&self, addr:&str) {
        if !self.lookup_table.keys().any(|val| *val == *addr) {
            panic!("\"{}\" is not an address!", addr);
        }
    }
    pub fn verify_free_addr(&self, addr:&str) {
        if self.lookup_table.keys().any(|val| *val == *addr) {
            panic!("\"{}\" address in use!", addr);
        }
    }
    
}
//--------------------------------------------------------
pub struct TouchOscButton {
    state : bool
}
impl TouchOscButton {
    pub fn new(state:bool) -> Self {
        TouchOscButton { 
            state: state 
        }
    }
    pub fn set_state (&mut self, arg:f32) {
        if arg > 0.0 {
            self.state = true;
        } else {
            self.state = false;
        }
    }
    pub fn state(&self) -> bool { // get
        return self.state;
    }

}
//--------------------------------------------------------
pub struct TouchOscFader {
    min : f32,
    max : f32,
    value : f32
}
impl TouchOscFader {
    pub fn new(min:f32, max:f32, value:f32) -> Self {
        TouchOscFader { 
            min   : min,
            max   : min,
            value : value //default
        }
    }
    pub fn set_min (&mut self, min:f32) { 
        self.min = min; 
    }
    pub fn set_max (&mut self, max:f32) { 
        self.max = max; 
    }
    pub fn set_value (&mut self, arg:f32) {
        self.value = self.range(arg); 
    }
    pub fn range(&self, arg:f32) -> f32 {
        return map_range(arg, 0.0, 1.0, self.min, self.max);
    }
    pub fn value(&self) -> f32 { // get
        return self.value;
    }
}
//--------------------------------------------------------
pub struct TouchOscGrid { 
    faders : Vec<TouchOscFader>,
    zero_index: bool
}
impl TouchOscGrid {
    pub fn new(size:usize, min:f32, max:f32, default:f32) -> Self {
        let mut faders =  Vec::new();
        for i in 0..size {
            faders.push(TouchOscFader::new(min, max, default));
        }
        TouchOscGrid {
            zero_index: false,
            faders
        }
    }
    pub fn set_zero_index(&mut self, bool:bool) {
        self.zero_index = bool;
    }
    pub fn value(&self, index:usize) -> f32 {
        if self.zero_index {
            return self.faders[index].value;
        } else {
            return self.faders[index-1].value;
        }
    }
}

//--------------------------------------------------------
pub struct TouchOscEncoder { 
    
}
impl TouchOscEncoder {
    pub fn new() -> Self {
        TouchOscEncoder {}
    }
}
//--------------------------------------------------------
pub struct TouchOscRadar { 
    
}
impl TouchOscRadar {
    pub fn new() -> Self {
        TouchOscRadar {}
    }
}
//--------------------------------------------------------
pub struct TouchOscRadial { 
    
}
impl TouchOscRadial {
    pub fn new() -> Self {
        TouchOscRadial {}
    }
}
//--------------------------------------------------------
pub struct TouchOscRadio { 
    
}
impl TouchOscRadio {
    pub fn new() -> Self {
        TouchOscRadio {}
    }
}
//--------------------------------------------------------
pub struct TouchOscXY { 
    
}
impl TouchOscXY {
    pub fn new() -> Self {
        TouchOscXY {}
    }
}