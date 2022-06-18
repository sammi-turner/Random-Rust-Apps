#![allow(unused_imports)]
#![allow(dead_code)]

extern crate ncurses;

use ncurses::*;
use ncurses::CURSOR_VISIBILITY::{CURSOR_INVISIBLE};

use std::*;
use std::io::*;
use std::time::SystemTime;
use std::fs::OpenOptions;

use rand::rngs::StdRng;
use rand::*;

// Seed the pseudo-random number generator with unix time
pub fn seed() {
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");
    let _rng = StdRng::seed_from_u64(d.as_secs());
}

// Generate a pseudo-random number between x and y
pub fn pseudo(x:i32, y:i32) -> i32 {
    return rand::thread_rng().gen_range(x..y + 1);
}

// Open the virtual terminal
pub fn vt_open() {
    initscr();
    raw();
    keypad(stdscr(), true);
}

// Close the virtual terminal on a user keypress, with the message x
pub fn vt_close(x:&str) {
    addstr(x);
    getch();
    endwin();
}

// Hides the virtual cursor
pub fn vt_cursor_off() {
    curs_set(CURSOR_INVISIBLE);
}

// Clears the virtual terminal
pub fn vt_cls() {
    clear();
}

// Obtains an i32 from a virtual terminal key press
pub fn vt_key_i32() -> i32 {
    let ch = getch();
    return ch;
}

// Obtains a char from a virtual terminal key press
pub fn vt_key_char() -> char {
    let ch = getch();
    let ch_u8 = ch as u8;
    let ch_char = ch_u8 as char;
    return ch_char;
}

// Displays the String x in the virtual terminal
pub fn vt_put_string(x:&mut String) {
    let slice:&str = &*x;
    addstr(slice);
    refresh();
}

// Displays the slice x in the virtual terminal
pub fn vt_put_slice(x: &str) {
    addstr(x);
    refresh();
}

// Write data to a file
pub fn file_write(path:&str, data:&str) {
    fs::write(path, data).expect("Unable to write file.");
}

// Append data to a file
pub fn file_append(path:&str, data:&str) {
    if file_exists(path) {
        let mut file = OpenOptions::new().append(true).open(path).expect("Unable to open file.");
        file.write_all(data.as_bytes()).expect("Write failed");
    }
    else {
        file_write(path, data);
    }
}

// Returns true if a file path exists
pub fn file_exists(path:&str) -> bool {
    return fs::metadata(path).is_ok();
}