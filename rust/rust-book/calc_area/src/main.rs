extern crate termion;

use std::io;
use std::error::Error;
use termion::clear;

fn display_main_menu() -> u32{
    let mut selection_valid = false;
    let mut input : u32 = 0;
    while ! selection_valid {
        let mut input_str = String::new();
        println!("**************************************************************************");
        println!("This program helps you calculate the area of various geometrical shapes.");
        println!("Press Ctrl+C at any time to quit.");
        println!("Type the number and press enter to calculate your choice:");
        println!("1: Calculate the area of a square");
        println!("2: Calculate the area of a rectangle");
        println!("3: Calculate the area of a circle");
        println!("0: Exit");
        println!("**************************************************************************");
        println!("Enter selection");

        match io::stdin().read_line(&mut input_str) {
            Ok(_) => {},
            Err(_) => {
                println!("{}", clear::All);

                println!("!! Couldn't read menu input !!");
                continue;
            },
        };

        input = match input_str.trim().parse() {
            Ok(input) => input,
            Err(e) => {
                println!("{}", clear::All);

                println!("!! {} !!", e.description());
                println!("!! Couldn't parse menu input !!");
                continue;
            },
        };

        if input >= 0 && input <= 3 {
            selection_valid = true;
        } else {
            println!("{}", clear::All);

            println!("!! You didn't enter a valid selection !!");
        }

    }
    
    return input;
}

fn main() {
    loop {
        println!("{}", clear::All);

        let input = display_main_menu();

        match input {
            0 => {
                println!("\nBye!");
                return;
            },
            _ => {
                println!("You entered {}", input);
            },
        };
    }

}
