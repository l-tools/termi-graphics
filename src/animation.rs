#[cfg(target_os = "unix")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "linux")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "windows")]
use crate::windows_cmd_sceme::*;

use crate::pixel_art::Screen;
use std::{thread,time};
use std::thread::JoinHandle;
///animation struct is used to connect between different screens and run them in a fixed speed.
pub struct Animation{
    roll:Vec<Screen>,
    fps:u8,
    length:u8,
    //anim_thread:&JoinHandle,
}
impl Animation{
    /// defines a new animation
    /// #Examples
    ///```
    /// use termi_graphics::pixel_art::{Screen,PixelColors};
    /// use termi_graphics::animation::Animation;
    /// let screen1 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let screen2 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let screen3 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let anim = Animation::new(vec![screen1,screen2,screen3],15,10);
    ///```
    ///creates a new animation made of 3 reoccuring screen running at 15 fps for 10 seconds.
    pub fn new(roll:Vec<Screen>,fps:u8,length:u8)->Animation{
        //let anim_thread = thread::spawn(||{});
        let an1 = Animation{roll,fps,length,/*anim_thread*/};
        an1
    }
    /// starts the animation
    /// #Examples
    ///```
    /// use termi_graphics::pixel_art::{Screen,PixelColors};
    /// use termi_graphics::animation::Animation;
    /// let screen1 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let screen2 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let screen3 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let anim = Animation::new(vec![screen1,screen2,screen3],15,10);
    /// anim.play();
    ///```
    /// running the 3 screen one after the other in rate of 15 screens per sec, for 10 seconds,
    /// then stops.
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
    /// stops animation 
    /// doesn't work well just yet - making threadpools in upcoming changes
    /// #Examples
    ///```
    /// use termi_graphics::pixel_art::{Screen,PixelColors};
    /// use termi_graphics::animation::Animation;
    /// let screen1 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let screen2 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let screen3 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let anim = Animation::new(vec![screen1,screen2,screen3],15,10);
    /// anim.play();
    /// anim.pause();
    ///```
    /// 
    pub fn pause(&self){
        print!("\u{1B}[?25l");
    }
}
