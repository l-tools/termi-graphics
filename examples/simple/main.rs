use termi_graphics::pixel_art::{Screen,PixelColors,Canvas};
use termi_graphics::animation::Animation;

fn main(){
    let mut vecan = Vec::new();
    let mut shape1 = Vec::new();
    shape1.push(vec![1,1,0,1,1]);
    shape1.push(vec![1,1,0,1,1]);
    shape1.push(vec![0,0,0,0,0]);
    shape1.push(vec![0,0,1,0,0]);
    shape1.push(vec![1,0,0,0,1]);
    shape1.push(vec![1,1,1,1,1]);
    let mut shape2 = Vec::new();
    shape2.push(vec![1,1,1,1,1]);
    shape2.push(vec![1,0,0,0,1]);
    shape2.push(vec![1,0,0,0,1]);
    shape2.push(vec![1,0,0,0,1]);
    shape2.push(vec![1,0,0,0,1]);
    shape2.push(vec![1,1,1,1,1]);
    let mut shape3 = Vec::new();
    shape3.push(vec![0,0,0,0,0]);
    shape3.push(vec![0,0,1,0,0]);
    shape3.push(vec![0,1,1,1,0]);
    shape3.push(vec![0,1,1,1,0]);
    shape3.push(vec![0,0,1,0,0]);
    shape3.push(vec![0,0,0,0,0]);
    // this is a rather basic and none usable example, if you would really like to learn how to use
    // the library you should go to oter examples
    let mut screen1 = Screen::new(25,25,PixelColors::Green).unwrap();
    let mut canvas1 = Canvas::new(17,17,PixelColors::Green).unwrap();
    canvas1.attach(&shape1,1,1).unwrap();
    canvas1.attach(&shape2,10,10).unwrap();
    screen1.attach(&canvas1,0,0).unwrap();
    let mut screen2 = Screen::new(25,25,PixelColors::Green).unwrap();
    let mut canvas2 = Canvas::new(17,17,PixelColors::Green).unwrap();
    canvas2.attach(&shape1,1,1).unwrap();
    canvas2.attach(&shape3,10,10).unwrap();
    screen2.attach(&canvas2,0,0).unwrap();
    &vecan.push(screen1);
    &vecan.push(screen2);
    let mut animation = Animation::new(vecan,15,10);
    animation.play();


}
