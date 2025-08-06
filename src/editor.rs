use crossterm::terminal::{enable_raw_mode,disable_raw_mode,Clear,ClearType};
use crossterm::event::{read, Event::Key ,KeyModifiers, KeyCode::{Char, Enter, Backspace}};
use crossterm::execute;
use crossterm::cursor::{MoveTo, EnableBlinking, DisableBlinking};
use std::io::{stdout, Write};
use std::option::Option::Some;

pub struct Editor {
    buffer: String,
}

impl Editor {
    pub fn default() -> Self{
        Editor{
            buffer: String::from(""),
        }
    }
    pub fn run(&mut self){
       Editor::init();
       Editor::main_loop(&mut self.buffer);
       Editor::close();
    }
    fn init(){
        let _ = enable_raw_mode();
        Editor::clear_screen();
        match execute!(stdout(),EnableBlinking){
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn close(){
        let _ = disable_raw_mode();
        Editor::clear_screen();
        match execute!(stdout(),DisableBlinking){
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn clear_screen(){
        match execute!(stdout(),Clear(ClearType::All)){
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
        match execute!(stdout(),MoveTo(0,0)){
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn main_loop(buffer: &mut String){
        loop{
            match read(){
                Ok(Key(event)) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL){
                        match event.code{
                            Char('q' | 'c') => {
                                break;
                            },
                            Char(c) => {
                                buffer.push(c);
                            },
                            _ => (),
                        }
                    }else{
                        match event.code{
                            Char(c)=>{
                                buffer.push(c);
                            },
                            Enter => {
                                buffer.push_str("\r\n");
                            },
                            Backspace => {
                                buffer.pop();
                                if buffer.chars().last() == Some('\r'){
                                    buffer.pop();
                                }
                            },
                            _ => (),
                        }
                    }
                },
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
            Editor::clear_screen();
            print!("{}",buffer);
            stdout().flush().unwrap();
        }
    }
}
