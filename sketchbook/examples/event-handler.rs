use nannou::prelude::*;

use nannou::prelude::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

// Define the custom struct.
struct MyStruct {
    tx: mpsc::Sender<i32>,
    counter: Arc<Mutex<i32>>,
}

impl MyStruct {
    // Creates a new instance of MyStruct
    fn new() -> (Self, Vec<mpsc::Receiver<i32>>) {
        let (tx, rx) = mpsc::channel();
        let counter = Arc::new(Mutex::new(0));
        (Self { tx, counter }, vec![rx])
    }

    // Method that emits an event
    pub fn update(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        self.tx.send(*counter).unwrap();
    }

    // Method that creates a new receiver
    pub fn new_listener(&self) -> mpsc::Receiver<i32> {
        let (tx, rx) = mpsc::channel();
        self.tx.clone().send(rx).unwrap();
        rx
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

//unique function
fn my_function(count: i32) {
    println!("my_function called with count: {}", count);
}

struct Model {
     my_struct: MyStruct,
    background_color: Rgb,
    handles: Vec<std::thread::JoinHandle<()>>,
}



fn model(app: &App) -> Model {

    let window_id = app
        .new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let win = app.window_rect();

    let (my_struct, receivers) = MyStruct::new();

    let mut handles = Vec::new();
    for rx in receivers {
        let handle = std::thread::spawn(move || {
            while let Ok(count) = rx.recv() {
                println!("Event emitted: {} times", count);
            }
        });
        handles.push(handle);
    }

     // You can add new listeners as needed
    let new_rx = my_struct.new_listener();
    let new_handle = std::thread::spawn(move || {
        while let Ok(count) = new_rx.recv() {
            println!("Event emitted: {} times (new listener)", count);
        }
    });
    handles.push(new_handle);



    Model {
        my_struct,
        background_color: Rgb::new(0.1, 0.1, 0.1),
        handles
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.my_struct.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(model.background_color);
    draw.to_frame(app, &frame).unwrap();
}
