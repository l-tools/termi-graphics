use crate::pixel_art::{Shape,PixelColors};


pub fn square(x:u8, color:PixelColors)-> Vec<Vec<u8>>{
    let mut col1;
    match color{
            PixelColors::Red => col1 = 1,
            PixelColors::Blue => col1 = 2,
            PixelColors::Green => col1 = 3,
            PixelColors::Yellow => col1 = 4,
            PixelColors::Black => col1 = 5,
            PixelColors::Cyan => col1 = 6,
            PixelColors::Magenta => col1 = 7,
            PixelColors::White => col1 = 8,
    }
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    if x ==1{
        &vec2.push(col1);
        &vec1.push(vec2);
        return vec1;
    }else if x==2{
        &vec2.push(col1);
        &vec2.push(col1);
        &vec1.push(vec2.clone());
        &vec1.push(vec2);
        return vec1;
    }
    for i in 0..x{
        &vec2.push(col1);
    }
    &vec1.push(vec2);
    vec2 = Vec::new();
    for q in 0..(x-2){
        &vec2.push(col1);
        for u in 0..(x-2){
            &vec2.push(0);
        }
        &vec2.push(col1);
        &vec1.push(vec2);
        vec2 = Vec::new();
    }
    vec2 = Vec::new();
    for i in 0..x{
        &vec2.push(col1);
    }
    &vec1.push(vec2);
    vec1
}
pub fn rectangle(x:u8,y:u8, color:PixelColors)-> Vec<Vec<u8>>{
    let mut col1;
    match color{
            PixelColors::Red => col1 = 1,
            PixelColors::Blue => col1 = 2,
            PixelColors::Green => col1 = 3,
            PixelColors::Yellow => col1 = 4,
            PixelColors::Black => col1 = 5,
            PixelColors::Cyan => col1 = 6,
            PixelColors::Magenta => col1 = 7,
            PixelColors::White => col1 = 8,
    }
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for i in 0..x{
        &vec2.push(col1);
    }
    &vec1.push(vec2);
    vec2 = Vec::new();
    for q in 0..(y-2){
        &vec2.push(col1);
        for u in 0..(x-2){
            &vec2.push(0);
        }
        &vec2.push(col1);
        &vec1.push(vec2);
        vec2 = Vec::new();
    }
    vec2 = Vec::new();
    for i in 0..x{
        &vec2.push(col1);
    }
    &vec1.push(vec2);
    vec1
}
pub fn line(len:u8,color:PixelColors, ver:bool)->Vec<Vec<u8>>{
    let mut col1;
    match color{
            PixelColors::Red => col1 = 1,
            PixelColors::Blue => col1 = 2,
            PixelColors::Green => col1 = 3,
            PixelColors::Yellow => col1 = 4,
            PixelColors::Black => col1 = 5,
            PixelColors::Cyan => col1 = 6,
            PixelColors::Magenta => col1 = 7,
            PixelColors::White => col1 = 8,
    }
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    if !ver{
        for i in 0..len{
            &vec2.push(col1);
        }
        &vec1.push(vec2);
    }else{
        &vec2.push(col1);
        for i in 0..len{
            &vec1.push(vec2.clone());   
        }
    }
    vec1
}
/*
pub fn triangle(base:u8,color:PxelColors)->Vec<Vec<u8>>{
    let mut col1;
    match color{
            PixelColors::Red => col1 = 1,
            PixelColors::Blue => col1 = 2,
            PixelColors::Green => col1 = 3,
            PixelColors::Yellow => col1 = 4,
            PixelColors::Black => col1 = 5,
            PixelColors::Cyan => col1 = 6,
            PixelColors::Magenta => col1 = 7,
            PixelColors::White => col1 = 8,
    }
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut loc = 0;
    let mut row = 0;
    if base%2==0{
        for x in 0..(base/2){
            &vec2.push(0);
        }
        &vec2.push(col1);
        &vec2.push(col1);
        loc = (base/2)-1;
        for x in ((base/2)+2)..base{
            &vec2.push(0);
        }
        &vec1.push(vec2);
    }else{
        for x in 0..(base/2){
            &vec2.push(0);
        }
        &vec2.push(col1);
        loc = base/2;
        for x in ((base/2)+2)..base{
            &vec2.push(0);
        }
        &vec1.push(vec2);
    }
}*/
