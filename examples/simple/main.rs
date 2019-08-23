use termiGraphics::pixel_art::{Screen,PixelColors};
use termiGraphics::animation::Animation;

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
    let mut screen1 = Screen::new(25,25,PixelColors::Green);
    screen1.attach(shape1.clone(),1,1);
    screen1.attach(shape2,10,10);
    let mut screen2 = Screen::new(25,25,PixelColors::Green);
    screen2.attach(shape1,1,1);
    screen2.attach(shape3,10,10);
    &vecan.push(screen1);
    &vecan.push(screen2);
    let mut animation = Animation::new(vecan,15,10);
    animation.play();


}
