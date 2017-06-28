extern crate termion;

use std::io;
use std::io::Read;
use std::error::Error;
use std::f64::consts::PI;
use termion::clear;

struct Square {
    side: f64
}

impl Square {
    fn display_area(&self){
        println!("The area of a square of side {} is {}", self.side,
             self.side*self.side);
    }
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle{
    fn display_area(&self){
        println!("The area of a rectangle of width {} and height {} is {}",
         self.width, self.height, self.width*self.height);
    }
}

struct Circle {
    radius : f64
}

impl Circle {
    fn display_area(&self){
        println!("The area of a circle of radius {} is {}", self.radius,
             self.radius*self.radius*(PI));
    }
}

fn display_float_prompt(prompt_str : &str) -> f64 {
    let mut input : f64 = 0.0;
    let mut entry_valid = false;

    while ! entry_valid {
        let mut input_str = String::new();
        println!("{}", prompt_str);
        
        match io::stdin().read_line(&mut input_str) {
            Ok(_) => {},
            Err(_) => {
                println!("{}", clear::All);

                println!("!! Couldn't read input !!");
                continue;
            },
        };

        input = match input_str.trim().parse() {
            Ok(input) => input,
            Err(e) => {
                println!("{}", clear::All);

                println!("!! {} !!", e.description());
                println!("!! Couldn't parse input !!");
                continue;
            },
        };

        entry_valid = true;
    }

    return input;
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

fn continue_prompt(){
    println!("Press any key to go back to the main menu");
    io::stdin().read(&mut [0u8]).unwrap();
}

fn main() {
    loop {
        println!("{}", clear::All);

        let input = display_main_menu();

        match input {
            1 => {
                println!("{}", clear::All);
                println!("The area of a square is equal to the length of one side times itself");
                let square_side = display_float_prompt("Enter the length of one side");
                let square = Square {side : square_side};
                square.display_area();
                continue_prompt();
            },
            2 => {
                println!("{}", clear::All);
                println!("The area of a rectangle is equal to the product of the length of its sides");
                let width = display_float_prompt("Enter the width of the rectangle");
                let height = display_float_prompt("Enter the height of the rectangle");
                let rect = Rectangle {width : width, height : height};
                rect.display_area();
                continue_prompt();
            },
            3 => {
                println!("{}", clear::All);
                println!("The area of a circle is equal to PI times the square of the radius");
                let radius = display_float_prompt("Enter the radius of the circle");
                let circle = Circle {radius : radius};
                circle.display_area();
                continue_prompt();
            },
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
