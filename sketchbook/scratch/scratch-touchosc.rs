use nannou::prelude::*;
use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum TouchOscInputType {
    Button, Fader, Grid, Encoder, Radar, Radial, Radio, XY
}

pub struct TouchOscClient {
    inputs_count : u16,
    inputs_map   : HashMap<u16, String>, //all touchosc inputs

    //touchosc input refs
    touchosc_faders  : HashMap<String, TouchOscFader>,
    touchosc_buttons : HashMap<String, TouchOscButton>,
}

impl TouchOscClient {
    pub fn new() -> Self {
        TouchOscClient {
            inputs_count     : 0,
            inputs_map       : HashMap::new(),
            touchosc_faders  : HashMap::new(),
            touchosc_buttons : HashMap::new()
        }
    }

    pub fn add_fader(&mut self, addr:&str, min:f32, max:f32, default:f32) {
        if self.addr_inuse(addr) {
            panic!("address \"{}\" already used!", addr);
        } else {
            let fader = TouchOscFader::new(min, max, default);
            self.touchosc_faders.insert((&addr).to_string(), fader);
            self.inputs_map.insert(self.inputs_count, (&addr).to_string());
            self.inputs_count = self.inputs_count + 1;
            println!("Count = {}", self.inputs_count);
        }
    }

    pub fn add_xy(&mut self, path:&str, min:f32, max:f32, default:f32) {
       // self.touchosc_xys.insert(path.to_string(), TouchOscXY::new(path, min, max, default));
    }

    pub fn add_button(&mut self, addr:&str, default:bool) {
        if self.addr_inuse(addr) {
            panic!("address \"{}\" already used!", addr);
        } else {
            let button = TouchOscButton::new(default);
            self.touchosc_buttons.insert((&addr).to_string(), button);
            self.inputs_map.insert(self.inputs_count, (&addr).to_string());
            self.inputs_count = self.inputs_count + 1;
            println!("Count = {}", self.inputs_count);
        }
    }

    pub fn addr_inuse(&self, addr:&str) -> bool {
        return self.inputs_map.values().any(|val| *val == *addr);
    }
    
}


fn main() {

    let mut client = TouchOscClient::new();

    let fader1 = client.add_fader("/fader1", 0.0, 1.0, 1.0);
    let fader2 = client.add_fader("/fader2", 0.0, 1.0, 1.0);

    let button1 = client.add_button("/button1", true);
    let button2 = client.add_button("/button2", true);


    // let fader = TouchOscInput::new("/grid/rows", TouchOscInputType::Fader { value: 0.0 });
    // let button = TouchOscInput::new("/grid/rows", TouchOscInputType::Button { state: false });

    // let value = fader.get(true);
    // let state = button.get(false);

    // let fader_value = match &value {
    //     TouchOscValue::Fader(x)  => *x,
    //     TouchOscValue::Button(b) => *b,
    //     _ => ()
    //     // TouchOscValue::Button(x) => *x,
    // };

    // let button_state = match &state {
    //     TouchOscValue::Fader(value)  => *value,
    //     TouchOscValue::Button(state) => *state,
    // };

    //println!("{}", fader_value);
    // println!("{}", button_state);
    // nannou::app(model).update(update).run();
}

//--------------------------------------------------------


//--------------------------------------------------------
pub struct TouchOscInput {
    touchosc_input_addr : String,
    touchosc_input_type : String
}
impl TouchOscInput  {
    pub fn new(touchosc_input_addr:&str, touchosc_input_type:&str) -> Self {
        TouchOscInput { 
            touchosc_input_addr: touchosc_input_addr.to_string(),
            touchosc_input_type: touchosc_input_type.to_string()
        }
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