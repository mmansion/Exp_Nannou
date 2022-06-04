use nannou::prelude::*;

//--------------------------------------------------------
fn main() {

    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    // assert!(re.is_match("2014-01-01"));
    let addr = "/grid";

    match addr {
        "/grid/1" => { println!("matched /grid/1") }
        _ => { println!("no match") }
    }

    /*    
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    */
}