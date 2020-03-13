extern crate termion;

mod services;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let cur_line = 1;
    let mut app = services::lib::App {stdout, stdin, cur_line};

    write!(app.stdout, "{}",
           termion::clear::All).unwrap();
    app.stdout.flush().unwrap();

    services::lib::typing_effect(&mut app,
                                 String::from("Wake up N..."));
    services::lib::typing_effect(&mut app,
                                 String::from("Wait a minute, who are you?"));

    for c in app.stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _              => continue,
        }
        app.stdout.flush().unwrap();
    }

    write!(app.stdout, "{}{}", termion::clear::All, termion::cursor::Show).unwrap();
}