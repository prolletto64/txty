use crossterm::terminal::{enable_raw_mode,disable_raw_mode,Clear,ClearType};
use crossterm::event::{read, Event::Key ,KeyModifiers, KeyCode::{Char, Enter}};
use crossterm::execute;
use crossterm::cursor::{MoveTo, EnableBlinking, DisableBlinking};
use std::io::{stdout, Write};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self{
        Editor{}
    }
    pub fn run(&self){
       Editor::init();
       Editor::main_loop();
       Editor::close();
    }
    fn init(){
        let _ = enable_raw_mode();
        Editor::clear_screen();
        match execute!(stdout(),EnableBlinking){
            Ok() => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn close(){
        let _ = disable_raw_mode();
        Editor::clear_screen();
        match execute!(stdout(),DisableBlinking){
            Ok() => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn clear_screen(){
        match execute!(stdout(),Clear(ClearType::All)){
            Ok() => (),
            Err(err) => panic!("{err}"),
        }
        match execute!(stdout(),MoveTo(0,0)){
            Ok() => (),
            Err(err) => panic!("{err}"),
        }
    }
    fn main_loop(){
        loop{
            match read(){
                Ok(Key(event)) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL){
                        match event.code{
                            Char('q' | 'c') => {
                                break;
                            },
                            Char(c) => {
                                println!("pressed ctrl-{c}\r");
                            },
                            _ => (),
                        }
                    }else{
                        match event.code{
                            Char(c)=>{
                                print!("{c}");
                            },
                            Enter => {
                                print!("\r\n");
                            },
                            _ => (),
                        }
                    }
                },
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
            stdout().flush().unwrap();
        }
    }
}
