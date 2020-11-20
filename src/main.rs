extern crate pancurses;

use std::env;

use pancurses::{endwin, initscr, noecho, Input};

// Creates a secondary loop to collect and display search input
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
    let output = args[1].to_owned() + "\n";
    let window = initscr();
    window.printw(&output);
    window.refresh();
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            // q - Quit
            Some(Input::Character('q')) => {
                break;
            }
            // / - Will be search, for now just starts text input for testing
            Some(Input::Character('/')) => {
                search(&window);
            }
            Some(Input::Character(_c)) => {
                // window.addch(c);
                window.addch('x');
            }
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }
    endwin();
}
