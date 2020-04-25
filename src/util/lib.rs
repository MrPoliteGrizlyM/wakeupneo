use std::{time, thread};
use termion::{color, style, clear};
use termion::raw::{RawTerminal};
use std::io::{Write, Stdout, Stdin};

const TYPE_RATE: u64 = 150;

pub struct App {
    pub stdout: RawTerminal<Stdout>,
    pub stdin: Stdin,
    pub cur_line: u16
}

pub fn typing_effect(app: &mut App, string: String) {
    let mut result = String::from("");

    for s in string.chars() {
        result.push(s);
        write!(app.stdout, "{cursor}{clear}{start}{result}{reset}",
               cursor = termion::cursor::Goto(1, app.cur_line),
               clear = clear::CurrentLine,
               start = color::Fg(color::LightGreen),
               result = result,
               reset = style::Reset).unwrap();
        app.stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(TYPE_RATE));
    }
    // Yeah, I know. I'm retarded
    app.cur_line += 1;
}