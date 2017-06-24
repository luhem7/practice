use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let sleep_time = time::Duration::from_millis(10);
    // let input_file_path = "./input/starting_seed.txt";

    println!("Running a simulation of the game of life!");

    // TODO Implement reading a seed from a file later
    // println!("Reading the input file");
    // let mut file = File::open("foo.txt")?;
    // let mut file = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    loop {
        println!("Press Ctrl+C at any time to quit");
        thread::sleep(sleep_time);
        print!("{}[2J", 27 as char);
    }
}


