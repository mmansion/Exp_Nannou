use nannou::prelude::*;
use nannou_osc as osc;
use std::collections::HashMap;

pub enum TouchOscInputType {
    Button(TouchOscButton),
    Fader(TouchOscFader),
    Grid,
    Encoder,
    Radar,
    Radial,
    Radio,
    XY(TouchOscXY),
}

pub struct TouchOscClient {
    osc_receiver: osc::Receiver,
    osc_inputs: HashMap<String, TouchOscInput>,
}

impl TouchOscClient {
    pub fn new(port: u16) -> Self {
        TouchOscClient {
            osc_receiver: osc::receiver(port).unwrap(), //Bind an `osc::Receiver` to a port.
            osc_inputs: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        // for (packet, addr) in self.osc_receiver.try_iter() {
        //     for msg in packet.into_msgs() {
        //         let args = msg.args.unwrap();

        //         match self.touchosc_faders.get_mut(&msg.addr) {
        //             Some(fader) => { fader.set (match &args[..] {
        //                 [osc::Type::Float(x)] => *x, _etc => fader.value });
        //             }, None => ( /*do nothing*/)
        //         }
        //         match self.touchosc_buttons.get_mut(&msg.addr) {
        //             Some(button) => { button.set (match &args[..] {
        //                 [osc::Type::Float(x)] => {  *x > 0.0 },
        //                 _etc => button.state });
        //             }, None => ( /*do nothing*/)
        //         }
        //     }
        // }
        // for (path, fader) in &self.touchosc_faders {
        //     //println!("{} {}", path, fader.value);
        // }
        // for (path, button) in &self.touchosc_buttons {
        //     //println!("{} {}", path, button.state);
        // }
    }

    pub fn add_fader(&mut self, addr: &str, min: f32, max: f32, default: f32) {

        //let osc_input = TouchOscInput::new(addr, TouchOscInputType::Fader);

        // osc_input.set()

        // self.osc_inputs.insert(addr.to_string(), TouchOscInput::new(addr, TouchOscInputType::Fader(TouchOscFader::new()) );

        //let fader = osc_inputs::new(&addr, min, max, default);
        // self.touchosc_faders.insert(addr, fader);

        //self.osc_inputs.insert(addr, TouchOscInputType::Fader(fader));

        // for (key, value) in &self.touchosc_inputs {
        // println!("{}: {:?}", key, value);
        // }
    }

    pub fn add_xy(&mut self, path: &str, min: f32, max: f32, default: f32) {
        // self.touchosc_xys.insert(path.to_string(), TouchOscXY::new(path, min, max, default));
    }

    pub fn add_button(&mut self, path: &str, default: bool) {
        //self.touchosc_buttons.insert(path.to_string(), TouchOscButton::new(path, default));
    }

    // touchosc input getters:

    pub fn fader(&self, addr: &str) -> f32 {
        // let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
        // for &book in &to_find {
        //     match book_reviews.get(book) {
        //         Some(review) => println!("{book}: {review}"),
        //         None => println!("{book} is unreviewed.")
        //     }
        // }
        // let fader = self.touchosc_hashmaps[addr].into(OscInputType::TouchOscFader);
        //let fader = &self.touchosc_inputs[addr];

        return 0.0;
    }

    pub fn button(&self, addr: &str) -> bool {
        return false;
        // return self.touchosc_buttons[addr].state;
    }
}
//--------------------------------------------------------

pub struct TouchOscInput {
    input_addr: String,
    // osc_input  : TouchOscInputType,
    // input_fader : Option<TouchOscFader>,
    // input_button: Option<TouchOscButton>,
    //TODO: input_grid  : Option<TouchOscGrid>,
    //TODO: input_encoder : Option<TouchOscEncoder>,
    //TODO: input_radar : Option<TouchOscRadar>,
    //TODO: input_radial : Option<TouchOscRadial>,
    //TODO: input_radio : Option<TouchOscRadio>,
    // input_xy : Option<TouchOscXY>,
}

impl TouchOscInput {
    pub fn new(input_addr: &str, input_type: TouchOscInputType) -> Self {
        //    let osc_input = match input_type {
        //         TouchOscInputType::Fader(x)  => { *x;  }
        //         TouchOscInputType::Button(x) => { *x; }
        //         TouchOscInputType::XY(x)     => { x  }
        //     };

        TouchOscInput {
            input_addr: input_addr.to_string(),
            // osc_input   :
        }
    }
    pub fn addr(&self) -> &str {
        return &self.input_addr;
    }
    pub fn set_range_min(&mut self, min: f32) {
        // if let Some(input_fader) {
        //     println!("SOME FADER YES");
        // }
        // if self.input_type == "fader" {
        // println!("{}", &self.input_fader.unwrap().range_min);
        // self.input_fader.set_range_min(min);
        // }
    }

    pub fn set(&self, value: Option<f32>, values: Option<Vec2>, state: Option<bool>) {}
}

//--------------------------------------------------------
pub struct TouchOscButton {
    state: bool, //true = ON, false = OFF
}

impl TouchOscButton {
    pub fn new() -> Self {
        TouchOscButton { state: false }
    }
    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }
    pub fn state(&self) -> bool {
        return self.state;
    }
}

//--------------------------------------------------------

pub struct TouchOscFader {
    pub range_min: f32,
    range_max: f32,
    osc_value: f32,
}

impl TouchOscFader {
    pub fn new() -> Self {
        TouchOscFader {
            range_min: 0.0,
            range_max: 1.0,
            osc_value: 0.0,
        }
    }
    pub fn set_range_min(&mut self, min: f32) {
        self.range_min = min;
    }
    pub fn set_range_max(&mut self, max: f32) {
        self.range_max = max;
    }
    pub fn set_value(&mut self, arg: f32) {
        self.osc_value = map_range(arg, 0.0, 1.0, self.range_min, self.range_max);
    }
    pub fn value(&self) -> f32 {
        // get
        return self.osc_value;
    }
}

//--------------------------------------------------------
pub struct TouchOscXY {
    range_min: f32,
    range_max: f32,
    osc_values: Vec2,
}

impl TouchOscXY {
    pub fn new() -> Self {
        TouchOscXY {
            range_min: 0.0,
            range_max: 1.0,
            osc_values: pt2(0.0, 0.0), //xy
        }
    }
    pub fn set_range_min(&mut self, min: f32) {
        self.range_min = min;
    }
    pub fn set_range_max(&mut self, max: f32) {
        self.range_max = max;
    }
    pub fn set_values(&mut self, args: Vec2) {
        self.osc_values.x = map_range(args.x, 0.0, 1.0, self.range_min, self.range_max);
        self.osc_values.y = map_range(args.y, 0.0, 1.0, self.range_min, self.range_max);
    }
    pub fn values(&self) -> Vec2 {
        return self.osc_values;
    }
}
