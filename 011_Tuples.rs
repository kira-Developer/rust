fn main(){
    let tup1 = (10 , 20 , 30 , 40 , "Rust" , 3.4 , true , (1,2,3));
    let tup2 = (1 , "Rust" , 3.5);
    let (a , b , c) = tup2;
    println!("{}" , tup1.3);
    println!("i can get tuple from inside tuple LOL {}" , (tup1.7).1);
    println!("a is {}" , a);
    println!("b is {}" ,b);
    println!("c is {}" , c);
}
