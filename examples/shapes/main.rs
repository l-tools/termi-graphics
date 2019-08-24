use termiGraphics::{pixel_art::{PixelColors,Screen},
                    shapes::{square,line},
                    animation::Animation};


fn main(){
    let mut vecan = Vec::new();
    let mut shape1 = Vec::new();
    //one way to make a smily face
    shape1.push(vec![2,2,0,2,2]);
    shape1.push(vec![2,2,0,2,2]);
    shape1.push(vec![0,0,0,0,0]);
    shape1.push(vec![0,0,1,0,0]);
    shape1.push(vec![8,0,0,0,8]);
    shape1.push(vec![8,8,8,8,8]);
    //another
    let mut shape2 =square(2,PixelColors::Blue) ;
    let mut shape3 =line(5,PixelColors::White,false) ;
    let mut shape4 =square(1,PixelColors::Red);
    let mut vecer = Vec::new();
    vecer.push(vec![8,0,0,0,8]);
     
    let mut screen1 = Screen::new(25,25,PixelColors::Black);
    screen1.attach(shape1,1,1);
    screen1.attach(shape2.clone(),10,10);
    screen1.attach(shape2,13,10);
    screen1.attach(shape4,12,13);
    screen1.attach(vecer,10,14);
    screen1.attach(shape3,10,15);
    &vecan.push(screen1);
    let mut animation = Animation::new(vecan,15,10);
    animation.play();
}
