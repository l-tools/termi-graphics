# termiGraphics
## What is it and what is it for?

#### termiGraphics is a rust crate(library) that should be used to build terminal based graphics and graphical interfaces.
#### termiGraphics offers low level graphics desgin options using canvases and screens, and the ability to run them consecutively with the Animation struct.
#### termiGraphics works best(for now) with the ubuntu terminal coloring scheme, but it still may work on some windows and mac computers.
#### There will soon be other crates designed to create other lower or higher level graphical interfaces on other places other then the CLI.

## How to install it?

### you can use cargo:
```console
:~$ cargo install termiGraphics
```
### or you can clone this repository:
```console
:~$ git clone https://github.com/l-tools/termiGraphics
```

## How to use it?

### First use import the crate:
```rust
extern crate termiGraphics;  
use termiGraphics::pixel_art; \\that is the basic part of the crate  
```

### There are 2 ways:
### The first one is to create the pixel layouts(2-dim vectors in our case) by yourself like this:
```rust
fn main(){                                                             
	let mut shape1 = Vec::new();                                       
	//smily face                                         
	shape1.push(vec![2,2,0,2,2]);                                      
	shape1.push(vec![2,2,0,2,2]);                                      
	shape1.push(vec![0,0,0,0,0]);                                          
	shape1.push(vec![0,0,1,0,0]);                                          
	shape1.push(vec![8,0,0,0,8]);                                          
	shape1.push(vec![8,8,8,8,8]); 
	screen1.attach_pixels(&shape1,1,1).unwrap();
	//simply print it once:
	screen1.print_screen();
	//or present it by animation:
	let vecan = vec![screen1];
	use termiGraphics::animation::Animation;
	let anim = Animation::new(vecan,15,3);
}
```
### A rather tedious way, so there is a second way:
```rust
fn main(){                                                             
	use termiGraphics::Shapes::{rectangle,line,sqaure,triangle};
	let shape2 =square(2,PixelColors::Blue).unwrap() ;                 
	let shape3 =line(5,PixelColors::White,false).unwrap();             
	let shape4 =square(1,PixelColors::Red).unwrap();                       
	let shape5 =rectangle(9,10,PixelColors::Magenta).unwrap();             
	let shape6 =triangle(9,PixelColors::Magenta).unwrap();                 
    	let mut vecer = Vec::new();                                            
    	vecer.push(vec![8,0,0,0,8]);                                           
    	let mut canvas1 = Canvas::new(30,30,PixelColors::Black).unwrap();      
    	let mut screen1 = Screen::new(35,35,PixelColors::Black).unwrap();   
	canvas1.attach(&shape5,8,8).unwrap();                              
    	canvas1.attach(&shape2,10,10).unwrap();                                
    	canvas1.attach(&shape2,13,10).unwrap();                                
    	canvas1.attach(&shape4,12,13).unwrap();                                
    	canvas1.attach(&vecer,10,14).unwrap();                                 
    	canvas1.attach(&shape3,10,15).unwrap();                                
    	canvas1.attach(&shape6,16,16).unwrap();   
	screen1.attach(&canvas1);
	//simply print it once:
	screen1.print_screen();
	//or present it by animation:
	let vecan = vec![screen1];
	use termiGraphics::animation::Animation;
	let anim = Animation::new(vecan,15,3);
}
```
#### I guess that seems tedious as well but when you are building something more than just a smiley face, shapes and canvases are someting much easier to design and follow then mere pixel vectors.

### If you want to checkout the crate further that you should take a look in the [examples](https://github.com/l-tools/termiGrapichs/examples) folder.


## More will be coming soon - in this crate and in others!
