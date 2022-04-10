struct Rectangle {
    width:u32,
    height:u32
}
impl Rectangle{
    fn print_description(&self){
        println!("Rectangle {} x {}" , self.width , self.height);
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
}
fn main(){
    let my_rect = Rectangle {width: 5 , height: 5};
    my_rect.print_description();
    println!("Rectangle is square: {}" , my_rect.is_square());
}