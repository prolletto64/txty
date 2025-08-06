use crossterm::cursor::{DisableBlinking, EnableBlinking, MoveTo};
use crossterm::event::{
    read,
    Event::Key,
    KeyCode::{Backspace, Char, Enter},
    KeyModifiers,
};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::fs::File;
use std::io::{stdout, Write};
use std::option::Option::Some;

pub struct Editor {
    buffer: String,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            buffer: String::from(""),
        }
    }
    pub fn run(&mut self) {
        Editor::init();
        Editor::main_loop(&mut self.buffer);
        Editor::close();
    }
    fn init() {
        let _ = enable_raw_mode();
        Editor::clear_screen();
        match execute!(stdout(), EnableBlinking) {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn close() {
        let _ = disable_raw_mode();
        Editor::clear_screen();
        match execute!(stdout(), DisableBlinking) {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn clear_screen() {
        match execute!(stdout(), Clear(ClearType::All)) {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
        match execute!(stdout(), MoveTo(0, 0)) {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn main_loop(buffer: &mut String) {
        loop {
            match read() {
                Ok(Key(event)) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                        match event.code {
                            Char('q' | 'c') => {
                                break;
                            }
                            Char('s')=>{
                                match File::create("out.txt"){
                                    Ok(mut f) => {
                                        match f.write_all(buffer.as_bytes()){
                                            Ok(()) =>(),
                                            Err(err) => panic!("{err}")
                                        }
                                    }
                                    Err(err) => panic!("{err}"),
                                }
                            }
                            Char(c) => {
                                buffer.push(c);
                            }
                            _ => (),
                        }
                    } else {
                        match event.code {
                            Char(c) => {
                                buffer.push(c);
                            }
                            Enter => {
                                buffer.push_str("\r\n");
                            }
                            Backspace => {
                                buffer.pop();
                                if buffer.chars().last() == Some('\r') {
                                    buffer.pop();
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
            Editor::clear_screen();
            print!("{}", buffer);
            stdout().flush().unwrap();
        }
    }
}
