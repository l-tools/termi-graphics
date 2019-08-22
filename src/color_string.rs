#[cfg(target_os = "unix")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "linux")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "windows")]
use crate::windows_cmd_sceme::*;

pub struct ColoredString{
    string:String,
    color:String,
}
impl ColoredString{
    pub fn new(str1:&str)->ColoredString{
        let colored1 = ColoredString{
            string: str1.to_string(),
            color: White.to_string(),
        };
        colored1
    }
    pub fn blue(&mut self){
        self.color = Blue.to_string();
    }
    pub fn cprint(&self){
        print!("{}{}{}",self.color,self.string,CLOSING_COLOR);
    }
    pub fn green(&mut self){
        self.color = Green.to_string();
    }
    pub fn yellow(&mut self){
        self.color = Yellow.to_string();
    }
    pub fn red(&mut self){
        self.color = Red.to_string();
    }
    pub fn black(&mut self){
        self.color = Black.to_string();
    }
    pub fn cyan(&mut self){
        self.color = Cyan.to_string();
    }
    pub fn magenta(&mut self){
        self.color = Magenta.to_string();
    }
    pub fn bold(&mut self){
        let str1 = Red.to_string();
        let str2 = Blue.to_string();
        let str3 = Green.to_string();
        let str4 = Yellow.to_string();
        let str5 = White.to_string();
        let str6 = Black.to_string();
        let str7 = Cyan.to_string();
        let str8 = Magenta.to_string();
        match &self.color{
            str1=> self.color = Reder.to_string(),
            str2=> self.color = Bluer.to_string(),
            str3=> self.color = Greener.to_string(),
            str4=> self.color = Yellower.to_string(),
            str5=> self.color = Whiter.to_string(),
            str6=> self.color = Blacker.to_string(),
            str7=> self.color = Cyaner.to_string(),
            str8=> self.color = Magentaer.to_string(),
        }
    }
    pub fn blink(&mut self){
        let str1 = Red.to_string();
        let str2 = Blue.to_string();
        let str3 = Green.to_string();
        let str4 = Yellow.to_string();
        let str5 = White.to_string();
        let str6 = Black.to_string();
        let str7 = Cyan.to_string();
        let str8 = Magenta.to_string();
        match &self.color{
            str1=> self.color = Redev.to_string(),
            str2=> self.color = Bluev.to_string(),
            str3=> self.color = Greenev.to_string(),
            str4=> self.color = Yellowev.to_string(),
            str5=> self.color = Whitev.to_string(),
            str6=> self.color = Blackev.to_string(),
            str7=> self.color = Cyanev.to_string(),
            str8=> self.color = Magentaev.to_string(),
        }
    }
    pub fn underline(&mut self){
        let str1 = Red.to_string();
        let str2 = Blue.to_string();
        let str3 = Green.to_string();
        let str4 = Yellow.to_string();
        let str5 = White.to_string();
        let str6 = Black.to_string();
        let str7 = Cyan.to_string();
        let str8 = Magenta.to_string();
        match &self.color{
            str1=> self.color = Redes.to_string(),
            str2=> self.color = Blues.to_string(),
            str3=> self.color = Greenes.to_string(),
            str4=> self.color = Yellowes.to_string(),
            str5=> self.color = Whites.to_string(),
            str6=> self.color = Blackes.to_string(),
            str7=> self.color = Cyanes.to_string(),
            str8=> self.color = Magentaes.to_string(),
        }
    }

}
