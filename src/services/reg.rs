use termion::input::TermRead;
use std::io::{Write, stdout, stdin};
use crate::services::lib::App;

pub fn reg(app: &mut App) {
    // let stdout = stdout();
    // let mut stdout = stdout.lock();
    // let stdin = stdin();
    // let mut stdin = stdin.lock();
    //
    // stdout.write_all(b"Username: ").unwrap();
    // stdout.flush().unwrap();
    //
    // let username = stdin.read_line();
    //
    // stdout.write_all(b"Password: ").unwrap();
    // stdout.flush().unwrap();
    //
    // let pass = stdin.read_passwd(&mut stdout);
    //
    // if let Ok(Some(pass)) = pass {
    //     stdout.write_all(b"\n").unwrap();
    // } else {
    //     stdout.write_all(b"Error\n").unwrap();
    // }



    // write!(app.stdout, "Username: ").unwrap();
    // app.stdout.flush().unwrap();
    // let mut username = String::new();
    // app.stdin.read_line(&mut username);
    //
    // write!(app.stdout, "Password: ").unwrap();
    // app.stdout.flush().unwrap();
    // let mut pass = String::new();
    // app.stdin.read_line(&mut pass);
}