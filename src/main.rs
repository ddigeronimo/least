extern crate pancurses;

use std::{
    cmp::min,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use pancurses::{endwin, initscr, noecho, Input};

// Uber-simple file loading
// Opens the specified file, reads with a BufReader, and returns a Vec of Strings
fn load_file(filename: &String) -> Vec<String> {
    let file: File = File::open(filename).expect("Unable to read file");
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
    let mut lines: Vec<String> = load_file(&filename).to_owned();

    let window: pancurses::Window = initscr();
    window.keypad(true);
    noecho();

    // The max x and y values represent the very edge of the window, so we can't actually access them
    let screen_height: i32 = window.get_max_y() - 1;
    let screen_width: i32 = window.get_max_x() - 1;
    let mut content_top: i32 = 0;
    let mut content_len = lines.len().to_owned() as i32;
    let mut content_bottom: i32 = min(screen_height - 1, content_len); // Make sure to reserve an additional line for program text
    let mut write_pos: i32;

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
                    write_pos = 0;
                    for i in content_top..content_bottom {
                        &window.printw(&lines[i as usize]);
                        write_pos += 1;
                        window.mv(write_pos, 0);
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
                    write_pos = 0;
                    for i in content_top..content_bottom {
                        &window.printw(&lines[i as usize]);
                        write_pos += 1;
                        window.mv(write_pos, 0);
                    }
                    &window.refresh();
                }
            }
            // d, PgDn - Move down half screen
            Some(Input::Character('d')) | Some(Input::KeyNPage) => {
                half_screen_down();
            }
            // u, PgUp - Move up half screen
            Some(Input::Character('u')) | Some(Input::KeyPPage) => {
                half_screen_up();
            }
            // o - Open new file
            Some(Input::Character('o')) => {
                // Move cursor to command section (Bottom right, minus 20 chars)
                let input_window_size: i32 = min(20, screen_width);
                window.mv(screen_height, screen_width - input_window_size);
                window.refresh();
                let mut input_str: String = String::new();
                let mut remaining_chars = input_window_size;
                loop {
                    match window.getch() {
                        Some(Input::Character('\n')) => {
                            endwin();
                            lines = load_file(&input_str);
                            content_len = lines.len() as i32;
                            content_bottom = min(screen_height - 1, content_len);
                            window.clear();
                            for i in 0..content_bottom {
                                window.printw(&lines[i as usize]);
                                window.mv(i + 1, 0);
                            }
                            window.refresh();
                            break;
                        }
                        // Pancurses doens't detect backspace on all platforms as KeyBackspace, so catch the raw char codes
                        Some(Input::Character('\u{7f}')) | Some(Input::Character('\u{8f}')) => {
                            remaining_chars += 1;
                            input_str.pop();
                            window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                            window.delch();
                            window.refresh();
                        }
                        Some(Input::Character(c)) => {
                            if remaining_chars > 0 {
                                remaining_chars -= 1;
                                input_str.push(c);
                                window.mvaddch(screen_height, screen_width - remaining_chars, c);
                                window.refresh();
                            }
                        }
                        Some(_input) => (),
                        None => (),
                    }
                }
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

/*
                                Window Sizing & Coordinates
                 0,0-----------------------------------------------------0,s_w
                   |                                                     |
                   |                                                     |
                   |                                                     |
                   |                                                     |
                   |                                                     |
                   |                                                     |
                   |                                                     |
                   |                                                     |
                   |                                                     |
                   |                                input:               |
               s_h,0-----------------------------------------------------s_h,s_w
*/
