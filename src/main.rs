extern crate pancurses;
extern crate shellexpand;

use std::{
    cmp::{max, min},
    env,
    fs::{metadata, File},
    io::{BufRead, BufReader},
};

use pancurses::{endwin, initscr, noecho, Input, Window};
use shellexpand::full;

const HELP_MESSAGE: &str = r#"
        :::       ::::::::::    :::     ::::::::::::::::::: 
        :+:       :+:         :+: :+:  :+:    :+:   :+:     
        +:+       +:+        +:+   +:+ +:+          +:+     
        +#+       +#++:++#  +#++:++#++:+#++:++#++   +#+     
        +#+       +#+       +#+     +#+       +#+   +#+     
        #+#       #+#       #+#     #+##+#    #+#   #+#     
        #######################     ### ########    ###     

                      © 2020 Dylan DiGeronimo

                Usage: least [-h, --help | filename]

                   Controls:
                       - q - Quit
                       - j, Down - Down one line
                       - k, Up - Up one line
                       - d, PgDn - Down half a screen
                       - u, PgUp - Up half a screen
                       - g - Jump to top of file
                       - o - Open a new file
                       - h - Open help screen
"#;

// Opens the specified file (expands tildes and vars) and reads to a vector with a BufReader
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

// Struct that holds properties of the current window for easy reference
// Originally, I was using variables scoped to the main method to track these values,
// but that lead to lots of borrowing issues + the inability to move logic out of the main method
struct WindowState {
    window: Window,
    lines: Vec<String>,
    screen_height: i32,
    screen_width: i32,
    content_top: i32,
    content_bottom: i32,
    content_len: i32,
}

impl WindowState {
    // Constructor for window state, takes in lines
    pub fn new(lines: Vec<String>) -> WindowState {
        let window: Window = initscr();
        window.keypad(true);
        noecho();
        // The max x and y values represent the very edge of the window, so we can't actually access them
        let screen_height: i32 = window.get_max_y() - 1;
        let screen_width: i32 = window.get_max_x() - 1;
        let content_len = lines.len() as i32;
        let content_top = 0;
        let content_bottom: i32 = min(screen_height - 1, content_len); // Make sure to reserve an additional line for program text
        return WindowState {
            window: window,
            lines,
            screen_height: screen_height,
            screen_width: screen_width,
            content_top: content_top,
            content_bottom: content_bottom,
            content_len: content_len,
        };
    }

    // Jump to a given line, as long as it's within the bounds of the current window
    // Effectively replaces the logic I had separated out into seperate code blocks for down one line, up one line, etc
    pub fn jump_to_line(self, n: &i32) -> WindowState {
        if *n >= 0 && *n <= self.content_len {
            self.window.clear();
            let new_content_top = *n;
            let new_content_bottom = min(*n + self.screen_height, self.content_len);
            let mut write_pos: i32 = 0;
            for i in new_content_top..new_content_bottom {
                self.window.printw(&self.lines[i as usize]);
                write_pos += 1;
                self.window.mv(write_pos, 0);
            }
            self.window.refresh();
            WindowState {
                window: self.window,
                lines: self.lines.to_vec(),
                screen_height: self.screen_height,
                screen_width: self.screen_width,
                content_top: new_content_top,
                content_bottom: new_content_bottom,
                content_len: self.content_len,
            }
        } else {
            self
        }
    }

    // Replace the contents of the current screen with the help menu
    pub fn help_menu(self) -> WindowState {
        self.window.clear();
        let help_vec: Vec<String> = vec![String::from(HELP_MESSAGE)];
        let help_len: i32 = help_vec.len() as i32;
        let new_content_bottom: i32 = min(self.screen_height, help_len);
        let mut write_pos: i32 = 0;
        for i in 0..new_content_bottom {
            self.window.printw(&help_vec[i as usize]);
            write_pos += 1;
            self.window.mv(write_pos, 0);
        }
        self.window.refresh();
        WindowState {
            window: self.window,
            lines: help_vec,
            screen_height: self.screen_height,
            screen_width: self.screen_width,
            content_top: 0,
            content_bottom: new_content_bottom,
            content_len: help_len,
        }
    }

    // Move the cursor to the input section (bottom right) and take user input
    // Once the user terminates input with enter, call load_file() and draw the new file on the screen
    pub fn open_file(self) -> WindowState {
        // Move cursor to command section (Bottom right, minus 20 chars)
        let input_window_size: i32 = min(20, self.screen_width);
        self.window
            .mv(self.screen_height, self.screen_width - input_window_size);
        self.window.refresh();
        let mut input_str: String = String::new();
        let mut remaining_chars = input_window_size;
        let new_lines: Vec<String>;
        let new_content_len: i32;
        let new_content_bottom: i32;
        loop {
            match self.window.getch() {
                Some(Input::Character('\n')) => {
                    new_lines = load_file(&input_str);
                    new_content_len = new_lines.len() as i32;
                    new_content_bottom = min(self.screen_height - 1, new_content_len);
                    self.window.clear();
                    for i in 0..new_content_bottom {
                        self.window.printw(&new_lines[i as usize]);
                        self.window.mv(i + 1, 0);
                    }
                    self.window.refresh();
                    break;
                }
                // Pancurses doens't detect backspace on all platforms as KeyBackspace, so catch the raw char codes
                Some(Input::Character('\u{7f}')) | Some(Input::Character('\u{8f}')) => {
                    remaining_chars += 1;
                    input_str.pop();
                    self.window
                        .mv(self.window.get_cur_y(), self.window.get_cur_x() - 1);
                    self.window.delch();
                    self.window.refresh();
                }
                Some(Input::Character(c)) => {
                    remaining_chars -= 1;
                    input_str.push(c);
                    if remaining_chars > 0 {
                        self.window.mvaddch(
                            self.screen_height,
                            self.screen_width - remaining_chars,
                            c,
                        );
                        self.window.refresh();
                    } else {
                        // Replace the last n characters of the input string with "...", where n is abs val of remaining_chars + 3, aka the overflow
                        // Ex: input_str = /Users/user/folder1/folder2/file (32 chars), new_display_str = ...der1/folder2/file
                        let mut new_display_str: String = input_str.clone();
                        new_display_str.replace_range(..remaining_chars.abs() as usize + 3, "...");
                        self.window.mvaddstr(
                            self.screen_height,
                            self.screen_width - input_window_size,
                            new_display_str,
                        );
                        self.window.mv(self.screen_height, self.screen_width);
                        self.window.refresh();
                    }
                }
                Some(_input) => (),
                None => (),
            }
        }
        WindowState {
            window: self.window,
            lines: new_lines,
            screen_height: self.screen_height,
            screen_width: self.screen_width,
            content_top: 0,
            content_bottom: new_content_bottom,
            content_len: new_content_len,
        }
    }
}

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
    let lines: Vec<String> = load_file(&filename);
    let mut state: WindowState = WindowState::new(lines);

    // jump_to_line() can also be used for the inital draw
    let init_pos: i32 = 0;
    state = state.jump_to_line(&init_pos);

    // Main control loop
    loop {
        match state.window.getch() {
            // q - Quit
            Some(Input::Character('q')) => {
                break;
            }
            Some(Input::Character('g')) => {
                state = state.jump_to_line(&init_pos);
            }
            // h - Open help page
            Some(Input::Character('h')) => {
                state = state.help_menu();
                // TODO: Press q to load original file
            }
            // j - Move down one line
            Some(Input::Character('j')) | Some(Input::KeyDown) => {
                let new_pos: i32 = state.content_top + 1;
                state = state.jump_to_line(&new_pos);
            }
            // k - Move up one line
            Some(Input::Character('k')) | Some(Input::KeyUp) => {
                let new_pos: i32 = state.content_top - 1;
                state = state.jump_to_line(&new_pos);
            }
            // d, PgDn - Move down half screen
            Some(Input::Character('d')) | Some(Input::KeyNPage) => {
                let half_screen_down: i32 = min(
                    state.content_top + (state.screen_height / 2),
                    state.content_len,
                );
                state = state.jump_to_line(&half_screen_down);
            }
            // u, PgUp - Move up half screen
            Some(Input::Character('u')) | Some(Input::KeyPPage) => {
                let half_screen_up: i32 = max(0, state.content_top - (state.screen_height / 2));
                state = state.jump_to_line(&half_screen_up);
            }
            // o - Open new file
            Some(Input::Character('o')) => {
                state = state.open_file();
            }
            // / - Will be search
            Some(Input::Character('/')) => {
                // TODO: Implement
            }
            // Handle resize
            Some(Input::KeyResize) => {
                // https://github.com/ihalila/pancurses/pull/65
                // TODO: Implement
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
