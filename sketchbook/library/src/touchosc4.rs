use nannou::prelude::*;
use nannou_osc as osc;
use derivative::Derivative;

use std::collections::HashMap;

pub enum TouchOscInputType {
    Button,
    Fader,
    Grid,
    Radial,
    Encoder,
    Radar,
    Radio,
    XY
}

// pub enum TouchOscInputOption {
//     Min(f32),
//     Max(f32),
//     Default(f32)
// }

pub struct TouchOscClient {
    osc_port: u16,
    osc_receiver: osc::Receiver,

    pub touchosc_faders  : HashMap<String, TouchOscFader>,
    pub touchosc_buttons : HashMap<String, TouchOscButton>
}

impl TouchOscClient {

    pub fn new(port:u16) -> Self {
        let osc_port = port;

        // Bind an `osc::Receiver` to a port.
        let osc_receiver = osc::receiver(osc_port).unwrap();

        let touchosc_faders = HashMap::new();
        let touchosc_buttons = HashMap::new();

        TouchOscClient {
            osc_port,
            osc_receiver,
            touchosc_faders,
            touchosc_buttons
        }
    }

    pub fn add(&mut self, input_type:TouchOscInputType, path:&str) {

    }

    pub fn update(&mut self) {
        for (packet, addr) in self.osc_receiver.try_iter() {
            for msg in packet.into_msgs() {
                let args = msg.args.unwrap();
                
                match self.touchosc_faders.get_mut(&msg.addr) {
                    Some(fader) => { fader.set (match &args[..] { 
                        [osc::Type::Float(x)] => *x, _etc => fader.value });
                    }, None => ( /*do nothing*/)
                }
                match self.touchosc_buttons.get_mut(&msg.addr) {
                    Some(button) => { button.set (match &args[..] { 
                        [osc::Type::Float(x)] => {  *x > 0.0 }, 
                        _etc => button.state });
                    }, None => ( /*do nothing*/)
                }
            }
        }
        for (path, fader) in &self.touchosc_faders {
            println!("{} {}", path, fader.value);
        }
        for (path, button) in &self.touchosc_buttons {
            println!("{} {}", path, button.state);
        }
    }

   
    pub fn add_fader(&mut self, path:&str, min:f32, max:f32, default:f32) {
        self.touchosc_faders.insert(path.to_string(), TouchOscFader::new(path, min, max, default));
    }

    pub fn add_button(&mut self, path:&str, default:bool) {
        self.touchosc_buttons.insert(path.to_string(), TouchOscButton::new(path, default));
    }

    pub fn add_xy(&mut self, path:&str) {

    }

    // input getters:

    // access fader by path-name
    pub fn fader(&self, path:&str) -> f32 {
        return self.touchosc_faders[path].value;
    }

    // access fader by path-name
    pub fn button(&self, path:&str) -> bool {
        return self.touchosc_buttons[path].state;
    }


}
//--------------------------------------------------------
pub struct TouchOscButton {
    pub path  : String,
    pub state : bool, //t = on, f = off
}

impl TouchOscButton {
    pub fn new(path:&str, default:bool) -> Self {
        let path  = path.to_string();
        let state = default;
        TouchOscButton {
             path, 
             state 
        }
    }
    pub fn set(&mut self, state:bool) {
        self.state = state;
    }
}

//--------------------------------------------------------
pub struct TouchOscFader {
    pub path  : String,
    pub value : f32, // represents first arg of osc msg
    min: f32,
    max: f32,
}

impl TouchOscFader {

    pub fn new(path:&str, min:f32, max:f32, default:f32) -> Self {
        let path = path.to_string();
        let value = default;
        let min = min;
        let max = max;

        TouchOscFader {
             path, 
             value,
             min,
             max
        }
    }

    pub fn set(&mut self, v:f32) {
        self.value = map_range(v, 0.0, 1.0, self.min, self.max);
    }
}