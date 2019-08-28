use crate::pixel_art::PixelColors;


pub fn square(x:u8, color:PixelColors)-> Option<Vec<Vec<u8>>>{
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
        return Some(vec1);
    }else if x==2{
        &vec2.push(col1);
        &vec2.push(col1);
        &vec1.push(vec2.clone());
        &vec1.push(vec2);
        return Some(vec1);
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
    Some(vec1)
}
pub fn rectangle(x:u8,y:u8, color:PixelColors)-> Option<Vec<Vec<u8>>>{
    if x==y{
        return square(x,color);
    }else if x==1{
        return line(y,color,true);
    }else if y==1{
        return line(x,color,false);
    }
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
    Some(vec1)
}
pub fn line(len:u8,color:PixelColors, ver:bool)->Option<Vec<Vec<u8>>>{
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
    Some(vec1)
}
pub fn triangle(base:u8,color:PixelColors)-> Option<Vec<Vec<u8>>>{
    if base%2==0{
        return None;
    }
    if base<5{
        return None;
    }
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
    let mut loc = base/2;
    let mut row = 0;
    while loc !=1{
        for x in 0..(base/2 - row){
            &vec2.push(0);
        }
        &vec2.push(col1);
        loc = base/2-row;
        for x in (((base/2)-row)..base-(base/2+2)+row){
            &vec2.push(0);
        }
        if row !=0{
            &vec2.push(col1);
        }
        &vec1.push(vec2);
        vec2 = Vec::new();
        row+=1;
    }
    vec2 = Vec::new();
    for x in 0..base{
            vec2.push(col1);
    }
    &vec1.push(vec2);
    Some(vec1)
}
