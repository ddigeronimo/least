# least


        :::       ::::::::::    :::     :::::::::::::::::::
        :+:       :+:         :+: :+:  :+:    :+:   :+:
        +:+       +:+        +:+   +:+ +:+          +:+
        +#+       +#++:++#  +#++:++#++:+#++:++#++   +#+
        +#+       +#+       +#+     +#+       +#+   +#+
        #+#       #+#       #+#     #+##+#    #+#   #+#
        #######################     ### ########    ###

                      © 2020 Dylan DiGeronimo


Least is a minimal, less-inspired paging application written in Rust, using [pancurses](https://crates.io/crates/pancurses).

Its intention is not to outperform less, either in terms of speed or features, although that would be a nice eventual goal. Instead, I'm creating it as a means of getting to learn an interesting technology that I don't get to use in my everyday work. As such, there'll probably be a good amount of code that's amateur-ish, but I'm going to try my best to follow Rust conventions as I learn them.

The functionality is also not intended to be a total clone of less, with the controls being geared toward my personal preferences. I'm also making a point of not looking at the less source code while I build this. Least isn't a direct translation of less from C to Rust - it's a reinterpretation that happens to be written in a different language. 

least is licensed under the MIT license, so feel free to download it, fork it, modify it, use it, love it, hate it, etc. You do you.

## Installation
### Requirements
- git
- A working Rust installation, including cargo
- An ncurses library for your OS

### Clone source from GitHub
    $ git clone https://github.com/ddigeronimo/least.git

### Build with cargo for personal use
    $ cargo build --release

For a development build, use
    
    $ cargo build

## Usage
After building, the executable will be located at `{installLocation}least/target/release/least`. Open a text file with `$ least {filename}`. Make sure to set least as executable, set its permissions as needed, and add it to your path (consider copying it to a `~/bin` directory).

## Implemented Features
- Controls:
    - q - Quit
    - h - Open help page
    - j, Down - Down one line 
    - k, Up - Up one line
    - d, PgDn - Down half a screen
    - u, PgUp - Up half a screen
    - g - Jump to top of file
    - o - Open a new file (expands tildes and environment variables with [shellexpand](https://crates.io/crates/shellexpand) and supports symlinks)
    - / - Search
    - ? - Reverse search
    - n - Jump to next search result
    - N - Jump to previous search result
- Command line flags:
    - -h, --help - Prints help page to command line
- Basic file loading
    - Error handling now added
    - Now the in-program file opening functionality allows filenames of practically indefinite length (in practice this is limited by the size of the i32 used to track input string length)
- Search highlighting

## Potential Future Features
- Search UX improvements:
    - "No results found" message
    - Prefix search term with "/"
    - Clear previous search input
- React to screen resize
- Jump to specific line
- Optimized file loading (if needed for performance reasons)
    - V1: Load lines as needed
    - V2: Add buffer functionality
- Number prefixing for movement commands

## Known Bugs
*All clear for now!*