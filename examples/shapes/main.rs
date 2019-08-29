use termi_graphics::{pixel_art::{PixelColors,Screen,Canvas},
                    shapes::{square,line,rectangle,triangle},
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
    let shape2 =square(2,PixelColors::Blue).unwrap() ;
    let shape3 =line(5,PixelColors::White,false).unwrap();
    let shape4 =square(1,PixelColors::Red).unwrap();
    let shape5 =rectangle(9,10,PixelColors::Magenta).unwrap();
    let shape6 =triangle(9,PixelColors::Magenta).unwrap();
    let mut vecer = Vec::new();
    vecer.push(vec![8,0,0,0,8]);
    let mut canvas1 = Canvas::new(30,30,PixelColors::Black).unwrap(); 
    let mut screen1 = Screen::new(35,35,PixelColors::Black).unwrap();
    screen1.attach_pixels(&shape1,1,1).unwrap();
    let mut screen2 = Screen::new(35,35,PixelColors::Black).unwrap();
    canvas1.attach(&shape5,8,8).unwrap();
    canvas1.attach(&shape2,10,10).unwrap();
    canvas1.attach(&shape2,13,10).unwrap();
    canvas1.attach(&shape4,12,13).unwrap();
    canvas1.attach(&vecer,10,14).unwrap();
    canvas1.attach(&shape3,10,15).unwrap();
    canvas1.attach(&shape6,16,16).unwrap();
    screen2.attach(&canvas1,0,0);

    &vecan.push(screen1);
    &vecan.push(screen2);
    let mut animation = Animation::new(vecan,15,10);
    animation.play();
}
