#[cfg(target_os = "unix")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "linux")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "windows")]
use crate::windows_cmd_sceme::*;

use crate::pixel_art::Screen;
use std::{thread, time};

pub struct Animation{
    roll:Vec<Screen>,
    fps:u8,
    length:u8,
}
impl Animation{
    pub fn new(roll:Vec<Screen>,fps:u8,length:u8)->Animation{
        let an1 = Animation{roll,fps,length};
        an1
    }
    pub fn play(&self){
        let mut cnt = 0usize;
        for i in 0..(self.fps*self.length/(self.roll.len() as u8)){
            self.roll[cnt].print_screen();
            print!("\x1B[2J");
            thread::sleep(time::Duration::from_millis(1000/self.fps as u64));
            cnt+=1;
            if cnt == self.roll.len(){
                cnt = 0;
            }
        }
        self.roll[cnt].print_screen();
        
    }
    pub fn pause(&self){
        print!("\u{1B}[?25l");
    }
}
