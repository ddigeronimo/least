# least

Least is a minimal, less-esque paging application written in Rust, using [pancurses](https://crates.io/crates/pancurses).

Its intention is not to outperform less, either in terms of speed or features, although that would be a nice eventual goal. Instead, I'm creating it as a means of getting to learn an interesting technology that I don't get to use in my everyday work. As such, there'll probably be a good amount of code that's amateur-ish, but I'm going to try my best to follow Rust conventions as I learn them.

The functionality is also not intended to be a total clone of less, with the controls being geared toward my personal preferences. I'm also making a point of not looking at the less source code while I build this - it isn't a direct translation of less to Rust, but a reinterpretation that happens to be written in a different language. 

Licensed under the MIT license, so feel free to download it, fork it, modify it, use it, love it, hate it, etc. You do you.

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
After building, the executable will be located at `{installLocation}least/target/release/least`. (Once file loading is implemented) Open a text file with `$ least {filename}`. Make sure to set least as executable, set its permissions as needed, and add it to your path (consider copying it to a `~/bin` directory).

## Implemented Features
- Controls:
    - q - Quit

## Future Features
- Basic file loading
- More controls:
    - j - Down one line 
    - k - Up one line
    - d - Down half screen
    - u - Up half screen
    - h - Help page
- Command line flags:
    - -h, --help - Open directly to help page
- Jumping
- Optimized file loading
- Search functionality