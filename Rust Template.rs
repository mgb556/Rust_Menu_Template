//////////////////////////////////////////////////////////////////////////////////
// Rust Template: 5 option menu
// 
// Author:
// Date Created:
// Date Modified: 
// Purpose:
//
//
//
//
//////////////////////////////////////////////////////////////////////////////////

use std::io;

fn main() {
    // Original Option
    let mut ogop = true;

    // Menu loop
    while ogop {

        // Menu
        println!("Enter your selection");
        println!("1.");
        println!("2.");
        println!("3.");
        println!("4.");
        println!("5.");

        let mut input = String::new();

        // Reads your input and assigns it to input, crashes program if failure
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Logic for what options you want in your menu
        match input.trim() {
            "1" => {
                first_op();
            }
            "2" => {
                second_op();
            }
            "3" => {
                third_op();
            }
            "4" => {
                fourth_op();
            }
            "5" => {
                fifth_op();
            }
            other => {
                println!("Error selecting option, crashing program T_T");
            }
        }

        println!("Continue? (true/false)");

        let mut decision = String::new();

        io::stdin().read_line(&mut decision).expect("Failed to read input");

        // Decide if you want to continue from the original menu or exit the program
        ogop = decision.trim().to_lowercase().parse::<bool>().unwrap_or(false);
    }
}

// Functions for your menu options

fn first_op() {

}

fn second_op() {

}

fn third_op() {

}

fn fourth_op() {

}

fn fifth_op() {

}