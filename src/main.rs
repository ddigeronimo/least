extern crate pancurses;

use std::{
    env, fs,
    io::{BufRead, BufReader},
};

use pancurses::{endwin, initscr, noecho, Input};

// Uber-simple file loading
// Opens the specified file and reads and returns each line with a BufReader
fn load_file(filename: &String) -> Vec<String> {
    let file: fs::File = fs::File::open(filename).expect("Unable to read file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Failed to read line in file"))
        .collect()
}

// TODO: Implement
fn help_page() {}

// TODO: Implement
fn line_down() {}

// TODO: Implement
fn line_up() {}

// TODO: Implement
fn half_screen_down() {}

// TODO: Implement
fn half_screen_up() {}

// Creates a secondary loop to collect and display search input
// Takes in a pointer to the current pancurses Window
// TODO: Convert from temporary function to actual search
fn search(window: &pancurses::Window) {
    loop {
        match window.getch() {
            Some(Input::Character('/')) => {
                break;
            }
            Some(Input::Character(c)) => {
                window.addch(c);
            }
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }
}

// Main program loop
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args[1].to_owned();
    let lines: Vec<String> = load_file(&filename);
    for line in lines {
        println!("{:?}", line);
    }
    // let window = initscr();
    // window.printw(&filename);
    // window.refresh();
    // window.keypad(true);
    // noecho();
    // loop {
    //     match window.getch() {
    //         // q - Quit
    //         Some(Input::Character('q')) => {
    //             break;
    //         }
    //         // h - Open help page
    //         Some(Input::Character('h')) => {
    //             help_page();
    //         }
    //         // j - Move down one line
    //         Some(Input::Character('j')) => {
    //             line_down();
    //         }
    //         // k - Move up one line
    //         Some(Input::Character('k')) => {
    //             line_up();
    //         }
    //         // d - Move down half screen
    //         Some(Input::Character('d')) => {
    //             half_screen_down();
    //         }
    //         // u - Move up half screen
    //         Some(Input::Character('u')) => {
    //             half_screen_up();
    //         }
    //         // / - Will be search, for now just starts text input for testing
    //         Some(Input::Character('/')) => {
    //             search(&window);
    //         }
    //         Some(Input::Character(_c)) => {
    //             // window.addch(c);
    //             window.addch('x');
    //         }
    //         Some(input) => {
    //             window.addstr(&format!("{:?}", input));
    //         }
    //         None => (),
    //     }
    // }
    // endwin();
}
