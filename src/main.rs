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

// Given a vector of strings crawl over it and search for any occurences
// Returns a vector of tuples corresponding to the (line number, start character number, end character number)
fn search_scraper(lines: &Vec<String>, search_term: &String) -> Vec<(i32, i32, i32)> {
    let mut results: Vec<(i32, i32, i32)> = Vec::new();
    let mut line_number: i32 = 0;
    for line in lines {
        if line.contains(search_term) {
            // Get a vec of tuples of (starting index of substring, substring)
            let line_result_tuples: Vec<(usize, &str)> = line.match_indices(search_term).collect();
            // Filter that vec into just the starting indices and put them into a tuple with the line number and the ending index of the substring
            for t in line_result_tuples {
                let end_index: i32 = (t.0 + search_term.len()) as i32;
                results.append(&mut vec![(line_number, t.0 as i32, end_index)]);
            }
        }
        line_number += 1;
    }
    results
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
    search_results: Vec<(i32, i32, i32)>
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
        let search_results: Vec<(i32, i32, i32)> = Vec::new();
        return WindowState {
            window: window,
            lines,
            screen_height: screen_height,
            screen_width: screen_width,
            content_top: content_top,
            content_bottom: content_bottom,
            content_len: content_len,
            search_results: search_results,
        };
    }

    // Jump to a given line, as long as it's within the bounds of the current window
    // Effectively replaces the logic I had separated out into separate code blocks for down one line, up one line, etc
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
                search_results: self.search_results,
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
            search_results: self.search_results,
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
        let new_search_results: Vec<(i32, i32, i32)> = Vec::new();
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
            search_results: new_search_results,
        }
    }

    // Take user input to feed into the search scraper
    // If reverse is true, reverses the search_results
    pub fn search(self, reverse: bool) -> WindowState {
        // Move cursor to command section (Bottom right, minus 20 chars)
        let input_window_size: i32 = min(20, self.screen_width);
        self.window
            .mv(self.screen_height, self.screen_width - input_window_size);
        self.window.refresh();
        let mut input_str: String = String::new();
        let mut remaining_chars = input_window_size;
        let mut search_results: Vec<(i32, i32, i32)>;
        let mut new_state: WindowState;
        loop {
            match self.window.getch() {
                Some(Input::Character('\n')) => {
                    search_results = search_scraper(&self.lines, &input_str);
                    if reverse {
                        search_results.reverse();
                    }
                    new_state = self;
                    new_state.search_results = search_results;
                    new_state = new_state.jump_to_next_search_result();
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
                Some(input) => {
                    self.window.addstr(&format!("{:?}", input));
                }
                None => (),
            }
        }
        new_state
    }

    // Move the screen to the line of the next search result, and rotate the list forward
    pub fn jump_to_next_search_result(self) -> WindowState {
        if self.search_results.len() != 0 {
            let jump_line: i32 = self.search_results[0].0;
            let mut new_state: WindowState = self.jump_to_line(&jump_line);
            new_state.search_results.rotate_left(1);
            new_state.highlight_search_results()
        } else {
            self
        }
    }

    // Move the screen to the line of the last search result, and rotate the list backward
    pub fn jump_to_last_search_result(self) -> WindowState {
        if self.search_results.len() != 0 {
            let jump_line: i32;
            if self.search_results.len() > 1 {
                // self.search_results.len() - 2 corresponds to the previous search result before rotation of search_results
                jump_line = self.search_results[self.search_results.len() - 2].0;
                let mut new_state: WindowState = self.jump_to_line(&jump_line);
                new_state.search_results.rotate_right(1);
                new_state.highlight_search_results()
            } else {
                self
            }
        } else {
            self
        }
    }

    pub fn highlight_search_results(self) -> WindowState {
        for result in &self.search_results {
            // If a search result's line is currently within the display, highlight it
            if result.0 >= self.content_top && result.0 <= self.content_bottom {
                // Get the on-screen line number of the search result
                let line_offset: i32 = result.0 - self.content_top;
                self.window.mv(line_offset, 0);
                // Split the line the result is on into 3 chunks: pre-search result, search result, post-search result
                let line: String = self.lines[result.0 as usize].clone();
                let pre_chunk: String = line.chars().take(result.1 as usize).collect();
                let result_string: String = line.chars().skip(result.1 as usize).take((result.2 - result.1) as usize).collect();
                let post_chunk: String = line.chars().skip(result.2 as usize).take((line.len() + 1) - result.2 as usize).collect();
                self.window.addstr(pre_chunk);
                self.window.attrset(pancurses::COLOR_PAIR(2));
                self.window.addstr(result_string);
                self.window.attrset(pancurses::COLOR_PAIR(1));
                self.window.addstr(post_chunk);
                self.window.addch('\n');
            }
            self.window.refresh();
        }
        self
    }
}

// Main program logic
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
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

    // Setup colors
    pancurses::start_color();
    pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    pancurses::init_pair(2, pancurses::COLOR_BLACK, pancurses::COLOR_GREEN);

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
            // / - Search
            Some(Input::Character('/')) => {
                state = state.search(false);
            }
            // ? - Reverse search
            Some(Input::Character('?')) => {
                state = state.search(true);
            }
            // n - Jump to next search result
            Some(Input::Character('n')) => {
                state = state.jump_to_next_search_result();
            }
            // N - Jump to last search result
            Some(Input::Character('N')) => {
                state = state.jump_to_last_search_result();
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
