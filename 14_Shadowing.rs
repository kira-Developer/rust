fn main (){
    let mut x = 100;

    {
        // do some stuff with 50

        let x = 50;

        println!("x: {}", x)
    }

    // return x to 100 
    
    println!("x: {}" , x)
}
