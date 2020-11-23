extern crate pancurses;

use std::{
    cmp::{max, min},
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

// Main program logic
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args[1].to_owned();
    let lines: Vec<String> = load_file(&filename);

    let window: pancurses::Window = initscr();
    window.keypad(true);
    noecho();
    let screen_height: i32 = window.get_max_y();
    let screen_width: i32 = window.get_max_x();
    let mut content_top: i32 = 0;
    let content_len = lines.len() as i32;
    let mut content_bottom: i32 = min(screen_height - 1, content_len); // Make sure to reserve one line for program text
    for i in 0..content_bottom {
        window.printw(&lines[i as usize]);
        window.mv(i + 1, 0);
    }
    window.refresh();

    // Main control loop
    loop {
        match window.getch() {
            // q - Quit
            Some(Input::Character('q')) => {
                break;
            }
            // h - Open help page
            Some(Input::Character('h')) => {
                help_page();
            }
            // j - Move down one line
            Some(Input::Character('j')) | Some(Input::KeyDown) => {
                if content_bottom < content_len {
                    window.clear();
                    content_top += 1;
                    content_bottom += 1;
                    let mut write_pos: i32 = 0;
                    for i in content_top..content_bottom {
                        &window.printw(&lines[i as usize]);
                        window.mv(write_pos, 0);
                        write_pos += 1;
                    }
                    &window.refresh();
                }
            }
            // k - Move up one line
            Some(Input::Character('k')) | Some(Input::KeyUp) => {
                if content_top > 0 {
                    window.clear();
                    content_top -= 1;
                    content_bottom -= 1;
                    let mut write_pos: i32 = 0;
                    for i in content_top..content_bottom {
                        &window.printw(&lines[i as usize]);
                        window.mv(write_pos, 0);
                        write_pos += 1;
                    }
                    &window.refresh();
                }
            }
            // d - Move down half screen
            Some(Input::Character('d')) => {
                half_screen_down();
            }
            // u - Move up half screen
            Some(Input::Character('u')) => {
                half_screen_up();
            }
            // / - Will be search, for now just starts text input for testing
            Some(Input::Character('/')) => {
                search(&window);
            }
            // Any other keys - do nothing
            Some(_input) => (),
            None => (),
        }
    }
    endwin();
}
