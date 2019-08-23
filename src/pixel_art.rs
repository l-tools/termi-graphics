#[cfg(target_os = "unix")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "linux")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "windows")]
use crate::windows_cmd_sceme::*;

pub enum PixelColors{
    Red, 
    Green,
    Yellow,
    Blue,
    Black,
    Cyan,
    Magenta,
    White,
}
pub struct Shape{
    color: String,
    width: u8,
    height: u8,
    shape_vec:Vec<Vec<u8>>,
}
impl Shape{
    pub fn new(width:u8,height:u8,color1:&str)->Shape{
        let mut vec1 = Vec::new();
        let mut shape_vec = Vec::new();
        for i in 0..height{
           for j in 0..width{
                &vec1.push(0);
           } 
           &shape_vec.push(vec1);
           vec1 = Vec::new();
        }
        let shape1 = Shape{
            color: color1.to_string(),
            width,
            height,
            shape_vec,
        };
        shape1
    }
    pub fn cprint(self){
        for row in &self.shape_vec{
            for sq in row{
                if *sq==1{
                    print!("{}  {}",self.color,CLOSING_COLOR);
                }else{
                    print!(" ");
                }
            }
            print!("\n");
        }


    }
}
pub struct Screen{
    width:u8,
    height:u8,
    color:PixelColors,
    pixel_vec:Vec<Vec<u8>>,
} 
impl Screen{
    pub fn new(width:u8,height:u8,color:PixelColors)->Screen{
        let mut vec1 = Vec::new();
        let mut pixel_vec = Vec::new();
        for i in 0..height{
           for j in 0..width{
                &vec1.push(0);
           } 
           &pixel_vec.push(vec1);
           vec1 = Vec::new();
        }
        let screen1 = Screen{width,height,color,
            pixel_vec, 
        };
        screen1
    }
    pub fn print_screen(&self){
        for y in &self.pixel_vec{
            for x in y{
                if *x==0{
                    match self.color{
                        PixelColors::Red=>print!("{}",RedScr),
                        PixelColors::Blue=>print!("{}",BlueScr),
                        PixelColors::Green=>print!("{}",GreenScr),
                        PixelColors::Yellow=>print!("{}",YellowScr),
                        PixelColors::Black=>print!("{}",BlackScr),
                        PixelColors::Magenta=>print!("{}",MagentaScr),
                        PixelColors::Cyan=>print!("{}",CyanScr),
                        PixelColors::White=>print!("{}",WhiteScr),
                    }
                }else{
                    match *x{
                        1=>print!("{}",RedScr),
                        2=>print!("{}",BlueScr),
                        3=>print!("{}",GreenScr),
                        4=>print!("{}",YellowScr),
                        5=>print!("{}",BlackScr),
                        6=>print!("{}",CyanScr),
                        7=>print!("{}",MagentaScr),
                        8=>print!("{}",WhiteScr),
                        _=>print!("impossible"),
                    }
                }
                print!("  ");
                print!("{}",CLOSING_COLOR);
            }
            print!("\n");
        }
    }
    pub fn attach(&mut self,shape:Vec<Vec<u8>>,x:u8,y:u8){
        if x>self.width{
            panic!("out of bounds! width is too big");
        }
        if x>self.height{
            panic!("out of bounds! height is too big");
        }
        let mut cor_x = (x) as usize;
        let mut cor_y = y as usize;
        for i in shape{
            for j in i{
               if j>0{
                    self.pixel_vec[cor_y][cor_x]=j;
               }else if j>8{
                println!("no such color");
               } 
               cor_x+=1;
            }
            cor_y+=1;
            cor_x = (x) as usize;
        }
    }
    pub fn dittach(&mut self,shape:Vec<Vec<u8>>,x:u8,y:u8){
        if x>self.width{
            panic!("out of bounds! width is too big");
        }
        if x>self.height{
            panic!("out of bounds! height is too big");
        }
        let mut cor_x = (x) as usize;
        let mut cor_y = y as usize;
        for i in shape{
            for j in i{
               if j>0{
                    self.pixel_vec[cor_y][cor_x]=0;
               }else if j>8{
                println!("no such color");
               } 
               cor_x+=1;
            }
            cor_y+=1;
            cor_x = (x) as usize;
        }
    }
}
