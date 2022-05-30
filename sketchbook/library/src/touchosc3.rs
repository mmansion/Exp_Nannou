use nannou::{prelude::*, lyon::geom::arrayvec::Array};
use nannou_osc as osc;
use derivative::Derivative;

use std::collections::HashMap;

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
                        [osc::Type::Float(x)] => *x, _etc => button.value });
                    }, None => ( /*do nothing*/)
                }
            }
        }
        for (path, fader) in &self.touchosc_faders {
            println!("{} {}", path, fader.value);
        }
        for (path, fader) in &self.touchosc_buttons {
            println!("{} {}", path, fader.value);
        }
    }

    pub fn fader(&self, path:&'static str) -> &TouchOscFader {//&'static str
        return &self.touchosc_faders[path];
    }

    pub fn button(&self, path:&'static str) -> &TouchOscButton {//&'static str
        return &self.touchosc_buttons[path];
    }

    pub fn add_fader(&mut self, path:&'static str) {
        self.touchosc_faders.insert(path.to_string(), TouchOscFader::new(path.to_string()));
    }

    pub fn add_button(&mut self, path:&'static str) {
        self.touchosc_buttons.insert(path.to_string(), TouchOscButton::new(path.to_string()));
    }


}
//--------------------------------------------------------
pub struct TouchOscButton {
    pub path  : String,
    pub value : f32, // represents first arg of osc msg
}

impl TouchOscButton {
    pub fn new(path:String) -> Self {
        let path = path;
        let value = 0.0;
        TouchOscButton {
             path, 
             value 
        }
    }
    pub fn set(&mut self, v:f32) {
        self.value = v;
    }
}

//--------------------------------------------------------
pub struct TouchOscFader {
    pub path  : String,
    pub value : f32, // represents first arg of osc msg
}

impl TouchOscFader {
    pub fn new(path:String) -> Self {
        let path = path;
        let value = 0.0;
        TouchOscFader {
             path, 
             value 
        }
    }
    pub fn set(&mut self, v:f32) {
        self.value = v;
    }
}