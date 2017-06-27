extern crate termion;

use std::io;
use std::error::Error;
use termion::clear;

struct Square {
    side: f64
}

struct Rectangle {
    width: f64,
    height: f64
}

struct Circle {
    radius : f64
}

fn display_menu(menu_desc: &str, menu_opts : Vec<&str>) -> u32 {
    let mut input : u32 = 0;
    let mut selection_valid = false;

    while ! selection_valid {
        let mut input_str = String::new();
        println!("**************************************************************************");
        println!("{}", menu_desc);
        println!("Press Ctrl+C at any time to quit.\nSelect one of the options below:");
        for (menu_num, menu_opt) in menu_opts.iter().enumerate() {
            println!("{}: {}",menu_num+1, menu_opt);
        }
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

        if input >= 1 && input <= (menu_opts.len() as u32) {
            selection_valid = true;
        } else {
            println!("{}", clear::All);

            println!("!! You didn't enter a valid selection !!");
        }
    }


    return input;
}

fn display_main_menu() -> u32{

    let menu_opts = vec!["Calculate the area of a square",
                        "Calculate the area of a rectangle",
                        "Calculate the area of a circle"];
    return display_menu("This program helps you calculate the area of various geometrical shapes.",
                 menu_opts);

}

fn main() {
    loop {
        println!("{}", clear::All);

        let input = display_main_menu();

        match input {
            // 1 => {
            //     println!("{}", clear::All);
            // },
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
