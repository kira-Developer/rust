struct Color {

    red : u8, // u8 0-255
    green : u8,
    blue : u8    
}
fn main(){
    // color : red // green // blue
    let mut bg = Color{red: 255 , green: 70 ,blue: 15};
    bg.green = 90;
    println!("{}, {}, {}" , bg.red , bg.green , bg.blue);
}
