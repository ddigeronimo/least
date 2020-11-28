extern crate pancurses;
extern crate shellexpand;

use std::{
    cmp::min,
    env,
    fs::{metadata, File},
    io::{BufRead, BufReader},
};

use pancurses::{endwin, initscr, noecho, Input};
use shellexpand::full;

const HELP_MESSAGE: &str = r#"
                :::       ::::::::::    :::     ::::::::::::::::::: 
                :+:       :+:         :+: :+:  :+:    :+:   :+:     
                +:+       +:+        +:+   +:+ +:+          +:+     
                +#+       +#++:++#  +#++:++#++:+#++:++#++   +#+     
                +#+       +#+       +#+     +#+       +#+   +#+     
                #+#       #+#       #+#     #+##+#    #+#   #+#     
                #######################     ### ########    ###     

                Usage: least [-h | --help] filename

                Controls:
                    - q - Quit
                    - j, Down - Down one line 
                    - k, Up - Up one line
                    - o - Open a new file
"#;

// Uber-simple file loading
// Opens the specified file (after expanding tildes and vars) and read to a vector with a BufReader
// Returns a Vec of Strings (errors return an error message to be displayed)
fn load_file(filename: &String) -> Vec<String> {
    let expanded_filename: String = full(&filename).unwrap().to_string();
    let ef_copy = expanded_filename.clone();
    let f = File::open(expanded_filename);
    if f.is_ok() {
        // If unwrapping the metadata for the file fails, fall back to the dir error
        let md = metadata(ef_copy).unwrap_or(metadata("/").unwrap());
        if md.is_file() {
            let reader = BufReader::new(f.unwrap());
            reader
                .lines()
                .map(|l| l.expect("Failed to read line in file"))
                .collect()
        } else {
            vec![String::from(format!(
                "Error: \"{}\" is a directory",
                filename
            ))]
        }
    } else {
        vec![String::from(format!(
            "Error: File \"{}\" does not exist",
            filename
        ))]
    }
}

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
    if args.len() < 2 || (args.len() == 3 && args[1].to_owned() != "-w") {
        println!("Usage: least [-h | --help] filename");
        return;
    }
    if args[1].to_owned() == "-h" || args[1].to_owned() == "--help" {
        println!("{}", HELP_MESSAGE);
        return;
    }

    let filename: String = args[1].to_owned();
    let mut lines: Vec<String> = load_file(&filename);

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
                window.clear();
                lines = vec![String::from(HELP_MESSAGE)];
                content_len = lines.len() as i32;
                content_bottom = min(screen_height - 1, content_len);
                window.clear();
                for i in 0..content_bottom {
                    window.printw(&lines[i as usize]);
                    window.mv(i + 1, 0);
                }
                window.refresh();
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
                            remaining_chars -= 1;
                            input_str.push(c);
                            if remaining_chars > 0 {
                                window.mvaddch(screen_height, screen_width - remaining_chars, c);
                                window.refresh();
                            } else {
                                // Replace the last n characters of the input string with "...", where n is abs val of remaining_chars + 3, aka the overflow
                                // Ex: input_str = /Users/user/folder1/folder2/file (32 chars), new_display_str = ...der1/folder2/file
                                let mut new_display_str: String = input_str.clone();
                                new_display_str.replace_range(..remaining_chars.abs() as usize + 3, "...");       
                                window.mvaddstr(screen_height, screen_width - input_window_size, new_display_str);
                                window.mv(screen_height, screen_width);
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
