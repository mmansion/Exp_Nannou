use nannou::prelude::*;
use nannou_osc as osc;
use std::collections::HashMap;
use regex::Regex;
use regex::escape;

#[derive(Debug)]
pub enum TouchOscInputType { Button, Fader, Grid, Encoder, Radar, Radial, Radio, XY }

pub struct TouchOscClient {
    osc_receiver : osc::Receiver,

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
    pub fn new(port:u16) -> Self {
        TouchOscClient {
            osc_receiver      : osc::receiver(port).unwrap(), //Bind `osc::Receiver` to port.
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
    pub fn update(&mut self) {
        for (packet, ip_addr) in self.osc_receiver.try_iter() {
            for msg in packet.into_msgs() {
                let args = msg.args.unwrap();
                let mut found_key = false;//TODO: remove this
                for (key, input_type) in &self.lookup_table {
                    if key == &msg.addr { //exact addr match
                        match &input_type {
                            TouchOscInputType::Button => { 
                                match self.touchosc_buttons.get_mut(&msg.addr) {
                                     Some(button) => { 
                                         button.set_state(match &args[..] { 
                                            [osc::Type::Float(x)] => *x, 
                                            _etc => button.value() }); 
                                            button.print(&msg.addr);
                                            found_key = true;
                                        }, None => ()
                                }
                            },
                            TouchOscInputType::Fader => { 
                                match self.touchosc_faders.get_mut(&msg.addr) {
                                     Some(fader) => { 
                                         fader.set_value(match &args[..] { 
                                            [osc::Type::Float(x)] => *x, 
                                            _etc => fader.value() }); 
                                            fader.print(&msg.addr);
                                            found_key = true;
                                        }, None => ()
                                }
                            },
                            TouchOscInputType::Encoder => { 
                                match self.touchosc_encoders.get_mut(&msg.addr) {
                                     Some(encoder) => { 
                                        encoder.set_value(match &args[..] { 
                                            [osc::Type::Float(x)] => *x, 
                                            _etc => encoder.value() }); 
                                            encoder.print(&msg.addr);
                                            found_key = true;
                                        }, None => ()
                                }
                            },
                            TouchOscInputType::XY => { 
                                match self.touchosc_xys.get_mut(&msg.addr) {
                                     Some(xy) => { 
                                        xy.set_values(match &args[..] { 
                                            [osc::Type::Float(x), osc::Type::Float(y)] => pt2(*x, *y), 
                                            _etc => xy.values() }); 
                                            xy.print(&msg.addr);
                                            found_key = true;
                                        }, None => ()
                                }
                            },
                            _ => { println!("Input not found") }
                        };
                    } else if Regex::new(format!(r#"{}/\d+"#, 
                        escape(key)).as_str()).unwrap().is_match(&msg.addr) && !found_key
                        { //partial addr match
                        match &input_type {
                            TouchOscInputType::Grid => { 
                                match self.touchosc_grids.get_mut(key) {
                                     Some(grid) => { 
                                        grid.set_value(&msg.addr, match &args[..] { 
                                            [osc::Type::Float(x)] => *x, 
                                            _etc => grid.value(&msg.addr) }); 
                                            grid.print(&msg.addr);
                                            found_key = true;
                                        }, None => ()
                                }
                            }, _ => { println!("Other") }
                        }
                    }
                }
            }
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
        println!("{}", addr);
        self.lookup_table.insert((&addr).to_string(),TouchOscInputType::Grid);
        self.touchosc_grids.insert((&addr).to_string(), TouchOscGrid::new(addr, size, min, max, default));
    }
    pub fn add_encoder(&mut self, addr:&str, min:f32, max:f32, default:f32) {
        self.verify_free_addr(addr);
        self.lookup_table.insert((&addr).to_string(),TouchOscInputType::Encoder);
        self.touchosc_encoders.insert((&addr).to_string(), TouchOscEncoder::new(min, max, default));
    }
    pub fn add_radar() {

    }
    pub fn add_radial() {

    }
    pub fn add_radio() {

    }
    pub fn add_xy(&mut self, addr:&str, min:f32, max:f32, default:f32) {
        self.verify_free_addr(addr);
        self.lookup_table.insert((&addr).to_string(), TouchOscInputType::XY);
        self.touchosc_xys.insert((&addr).to_string(), TouchOscXY::new(min, max, default));
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
    pub fn grid(&self, addr:&str) -> f32 {
        for (key, input_type) in &self.lookup_table {
            let re = Regex::new(format!(r#"{}/\d+"#, escape(key)).as_str()).unwrap();
            if re.is_match(addr) {
                return match &input_type {
                    TouchOscInputType::Grid => { 
                        self.touchosc_grids[key].value(addr)
                    }, _ => { 0.0 }
                }
            }
        } return 0.0;
    }
    pub fn encoder(&self, addr:&str) -> f32 {
        self.verify_has_addr(addr);
        for (key, input_type) in &self.lookup_table {
            if key == addr {
                return match &input_type { //verify correct type at addr
                    TouchOscInputType::Encoder => { self.touchosc_encoders[addr].value() },
                    _ => { 0.0 }
                };
            }
        } return 0.0;
    }
    pub fn xy(&self, addr:&str) -> Vec2 {
        self.verify_has_addr(addr);
        for (key, input_type) in &self.lookup_table {
            if key == addr {
                return match &input_type { //verify correct type at addr
                    TouchOscInputType::XY => { self.touchosc_xys[addr].values() },
                    _ => { pt2(0.0, 0.0) }
                };
            }
        } return pt2(0.0, 0.0);
    }
    pub fn verify_has_addr(&self, addr:&str) { //TODO: try contains?
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
    state : bool,
    value : f32
}
impl TouchOscButton {
    pub fn new(state:bool) -> Self {
        let value = match state { true => 1.0, _ => 0.0};
        TouchOscButton { 
            state: state,
            value
        }
    }
    pub fn print(&self, addr:&str) {
        println!("{} {}", addr, self.state);
    }
    pub fn set_state (&mut self, value:f32) {
        if value > 0.0 {
            self.state = true;
        } else {
            self.state = false;
        }
    }
    pub fn state(&self) -> bool { // get
        return self.state;
    }
    pub fn value(&self) -> f32 { // get
        return self.value;
    }
}
//--------------------------------------------------------
pub struct TouchOscFader {
    min : f32,
    max : f32,
    value : f32
}
impl TouchOscFader {
    pub fn new(min:f32, max:f32, default:f32) -> Self {
        TouchOscFader { 
            min   : min,
            max   : max,
            value : default
        }
    }
    pub fn print(&self, addr:&str) {
        println!("{} {}", addr, self.value);
    }
    pub fn set_min (&mut self, min:f32) { 
        self.min = min; 
    }
    pub fn set_max (&mut self, max:f32) { 
        self.max = max; 
    }
    pub fn set_value (&mut self, value:f32) {
        self.value = self.range(value); 
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
    base_addr : String,
    faders : HashMap<String, TouchOscFader>,
}
impl TouchOscGrid {
    pub fn new(base_addr:&str, size:usize, min:f32, max:f32, default:f32) -> Self { 
        let mut faders = HashMap::new();
        for i in 0..size {
            let addr = format!("{}/{}", base_addr, (i+1).to_string());
            faders.insert((&addr).to_string(), TouchOscFader::new(min, max, default));
        }
        TouchOscGrid {
            base_addr : base_addr.to_string(),
            faders
        }
    }
    pub fn print(&self, addr:&str) {
        if self.faders.contains_key(addr) {
            println!("{} {}", addr, self.faders[addr].value());
        }
    }
    pub fn base_addr(&self) -> &str {
        return &self.base_addr;
    }
    pub fn set_value (&mut self, addr:&str, value:f32) {
        if self.faders.contains_key(addr) {
            match self.faders.get_mut(addr) { Some(fader) => fader.set_value(value), None => () }
        } else {
            println!("cannot value on 'out of bounds' grid element: {}", addr);
        }
    }
    pub fn value(&self, addr:&str) -> f32 {
        return self.faders[addr].value();
    }
}

//--------------------------------------------------------
pub struct TouchOscEncoder { 
    min : f32,
    max : f32,
    value : f32
}
impl TouchOscEncoder {
    pub fn new(min:f32, max:f32, default:f32) -> Self {
        TouchOscEncoder { 
            min   : min,
            max   : max,
            value : default //default
        }
    }
    pub fn print(&self, addr:&str) {
        println!("{} {}", addr, self.value);
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
    min  : f32,
    max  : f32,
    values : Vec2
}
impl TouchOscXY {
    pub fn new(min:f32, max:f32, default:f32) -> Self {
        TouchOscXY {
             min  : min,
             max  : max,
             values : pt2(default, default) //xy
        }
    }
    pub fn print(&self, addr:&str) {
        println!( "{} {},{}", addr, self.values.x, self.values.y);
    }
    pub fn set_min(&mut self, min:f32) {
        self.min = min;
    }
    pub fn set_max(&mut self, max:f32) {
        self.max = max;
    }
    pub fn set_values(&mut self, args:Vec2) {
        self.values.x = self.range(args.x);
        self.values.y = self.range(args.y);
    }
    pub fn range(&self, arg:f32) -> f32 {
        return map_range(arg, 0.0, 1.0, self.min, self.max);
    }
    pub fn values(&self) -> Vec2 {
        return self.values;
    }
}