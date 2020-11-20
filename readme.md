# least

Least is a minimal paging application/less clone written in Rust, using [pancurses](https://crates.io/crates/pancurses).

Its intention is not to outperform less, either in terms of speed or features, although that would be a nice eventual goal. Instead, I'm creating it as a means of getting to learn an interesting technology that I don't get to use in my everyday work. As such, there'll probably be a good amount of code that's amateur-ish, but I'm going to try my best to follow Rust conventions as I learn them.

Licensed under the MIT license, so feel free to download it, fork it, modify it, use it, love it, hate it, etc. You do you.

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
    - h - Help menu
- Optimized file loading
- Search functionality