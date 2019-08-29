#[cfg(target_os = "unix")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "linux")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "windows")]
use crate::windows_cmd_sceme::*;
///the enum used to check and send colors
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
///the screen struct, used to attach canvases
pub struct Screen{
    width:u8,
    height:u8,
    color:PixelColors,
    pixel_vec:Vec<Vec<u8>>,
} 
//the new does'nt need to worry about negatives because it's unsigned but still about zeros...
impl Screen{
    /// defines a new screen
    /// #Examples
    ///```
    /// use termiGraphics::pixel_art::{Screen,PixelColors};
    /// let screen1 = Screen::new(25,25,PixelColors::Red).unwrap();
    ///```
    ///creates a new 25x25 blank red screen
    pub fn new(width:u8,height:u8,color:PixelColors)->Option<Screen>{
        if height == 0 || width == 0{
            return None;
        }
        let mut vec1 = Vec::new();
        let mut pixel_vec = Vec::new();
        for i in 0..height{
           for j in 0..width{
                &vec1.push(0);
           } 
           &pixel_vec.push(vec1);
           vec1 = Vec::new();
        }
        let screen1 = Screen{width,height,color,pixel_vec,};
        Some(screen1)
    }
    /// output function, should not usually be used, you should use animation.play()
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
    ///needed in case anyone is realy into vecs....
    pub fn attach_pixels(&mut self,shape:&Vec<Vec<u8>>,x:u8,y:u8)->Result<(),&str>{
        if x>self.width{
            return Err("out of bounds! width is too big");
        }
        if x>self.height{
            return Err("out of bounds! height is too big");
        }
        let mut cor_x = (x) as usize;
        let mut cor_y = y as usize;
        for i in shape{
            for j in i{
               if *j>0{
                    self.pixel_vec[cor_y][cor_x]=*j;
               }else if *j>8{
                return Err("no such color");
               } 
               cor_x+=1;
            }
            cor_y+=1;
            cor_x = (x) as usize;
        }
        Ok(())
    }
    ///attach takes a drawn upon canvas and attaches it to the screen at a given point
    /// #Examples
    ///```
    /// use termiGraphics::pixel_art::{Screen,PixelColors,Canvas};
    /// use termiGraphics::shapes::line;
    /// let mut screen1 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let mut canvas1 = Canvas::new(5,5,PixelColors::Red).unwrap();
    /// let ln1 = line(4,PixelColors::Green,false).unwrap();
    /// canvas1.attach(&ln1,1,1).unwrap();
    /// screen1.attach(&canvas1,10,10).unwrap();
    ///```
    ///creates a new 25x25 blank red screen
    pub fn attach(&mut self,canvas:&Canvas,x:u8,y:u8)->Result<(),&str>{
        if x>self.width{
            return Err("out of bounds! width is too big");
        }
        if x>self.height{
            return Err("out of bounds! height is too big");
        }
        let mut cor_x = (x) as usize;
        let mut cor_y = y as usize;
        let vec55 = &canvas.pixel_vec;
        for i in vec55{
            for j in i{
               if *j>0{
                    self.pixel_vec[cor_y][cor_x]=*j;
               }else if *j>8{
                return Err("no such color");
               } 
               cor_x+=1;
            }
            cor_y+=1;
            cor_x = (x) as usize;
        }
        Ok(())
    }
    ///dittach removes only the cavases belongings from the screen, it works like an earaser that
    ///you point to a certain spot and it earases in a fixed manor
    /// #Examples
    ///```
    /// use termiGraphics::pixel_art::{Screen,PixelColors,Canvas};
    /// use termiGraphics::shapes::line;
    /// let mut screen1 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let mut canvas1 = Canvas::new(5,5,PixelColors::Red).unwrap();
    /// let ln1 = line(4,PixelColors::Green,false).unwrap();
    /// canvas1.attach(&ln1,1,1).unwrap();
    /// screen1.attach(&canvas1,10,10).unwrap();
    /// screen1.dittach(&canvas1,10,10).unwrap();
    ///```
    pub fn dittach(&mut self,canvas:&Canvas,x:u8,y:u8)->Result<(),&str>{
        if x>self.width{
            return Err("out of bounds! width is too big");
        }
        if x>self.height{
            return Err("out of bounds! height is too big");
        }
        let mut cor_x = x as usize;
        let mut cor_y = y as usize;
        let vec55 = &canvas.pixel_vec;
        for i in vec55{
            for j in i{
               if *j>0{
                    self.pixel_vec[cor_y][cor_x]=0;
               }else if *j>8{
                return Err("no such color");
               } 
               cor_x+=1;
            }
            cor_y+=1;
            cor_x = x as usize;
        }
        Ok(())
    }
    ///dittach_pixels can remove a vec of pixels of the screen, again with the eareaser analogy
    /// #Examples
    ///```
    /// use termiGraphics::pixel_art::{Screen,PixelColors,Canvas};
    /// use termiGraphics::shapes::line;
    /// let mut screen1 = Screen::new(25,25,PixelColors::Red).unwrap();
    /// let mut canvas1 = Canvas::new(5,5,PixelColors::Red).unwrap();
    /// let ln1 = line(4,PixelColors::Green,false).unwrap();
    /// canvas1.attach(&ln1,1,1).unwrap();
    /// screen1.attach(&canvas1,10,10).unwrap();
    /// screen1.dittach_pixels(&ln1,11,11).unwrap();
    ///```
    pub fn dittach_pixels(&mut self,shape:&Vec<Vec<u8>>,x:u8,y:u8)->Result<(),&str>{
        if x>self.width{
            return Err("out of bounds! width is too big");
        }
        if x>self.height{
            return Err("out of bounds! height is too big");
        }
        let mut cor_x = x as usize;
        let mut cor_y = y as usize;
        for i in shape{
            for j in i{
               if *j>0{
                    self.pixel_vec[cor_y][cor_x]=0;
               }else if *j>8{
                return Err("no such color");
               } 
               cor_x+=1;
            }
            cor_y+=1;
            cor_x = x as usize;
        }
        Ok(())
    }
}
/// canvas struct,  to draw upon it and once it's done attach it to the screen
pub struct Canvas{
    width:u8,
    height:u8,
    color:PixelColors,
    pixel_vec:Vec<Vec<u8>>,
} 
impl Canvas{
    ///as the one with screen, canvas is a small screen.
    pub fn new(width:u8,height:u8,color:PixelColors)->Option<Canvas>{
        if height == 0 || width == 0{
            return None;
        }
        let mut vec1 = Vec::new();
        let mut pixel_vec = Vec::new();
        for i in 0..height{
           for j in 0..width{
                &vec1.push(0);
           } 
           &pixel_vec.push(vec1);
           vec1 = Vec::new();
        }
        let canvas1 = Canvas{width,height,color,pixel_vec,};
        Some(canvas1)
    }
    ///you should only be able to print a canvas in debug
    #[cfg(debug_assertions)]
    pub fn print_canvas(&self){
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
    ///as the one in screen(attach_pixels)
    pub fn attach(&mut self,shape:&Vec<Vec<u8>>,x:u8,y:u8)->Result<(),&str>{
        if x>self.width{
            return Err("out of bounds! width is too big");
        }
        if x>self.height{
            return Err("out of bounds! height is too big");
        }
        let mut cor_x = (x) as usize;
        let mut cor_y = y as usize;
        for i in shape{
            for j in i{
               if *j>0{
                    self.pixel_vec[cor_y][cor_x]=*j;
               }else if *j>8{
                return Err("no such color");
               } 
               cor_x+=1;
            }
            cor_y+=1;
            cor_x = (x) as usize;
        }
        Ok(())
    }
    ///as the one in screen(dittach_pixels)
    pub fn dittach(&mut self,shape:&Vec<Vec<u8>>,x:u8,y:u8)->Result<(),&str>{
        if x>self.width{
            return Err("out of bounds! width is too big");
        }
        if x>self.height{
            return Err("out of bounds! height is too big");
        }
        let mut cor_x = x as usize;
        let mut cor_y = y as usize;
        for i in shape{
            for j in i{
               if *j>0{
                    self.pixel_vec[cor_y][cor_x]=0;
               }else if *j>8{
                return Err("no such color");
               } 
               cor_x+=1;
            }
            cor_y+=1;
            cor_x = x as usize;
        }
        Ok(())
    }
}
