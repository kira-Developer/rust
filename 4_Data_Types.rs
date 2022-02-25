/*
===============================
+ Length  + Signed + Unsigned +
+ 8-bit   + i8 	   + u8       +
+ 16-bit  + i16    + u16      +
+ 32-bit  + i32    + u32      +
+ 64-bit  + i64    + u64      +
+ arch 	  + isize  + usize    +
===============================



==========================================
+ Number literals   +	 Example         +
+ Decimal 	        +     98_222         +   
+ Hex 	            +     0xff           +
+ Octal 	        +     0o77           +
+ Binary 	        +     0b1111_0000    +
+ Byte              +     (u8 only) b'A' +
==========================================
*/


fn main ()
{
    let x: u64 = 45; // u64
    let f: f32  = 3.5; //f32 
    let b:bool = true; // boolen
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A' ;
    let name = "kira" ;
    let tup = (500, 6.4, 1);
    let tup_1: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}",x);
    println!("{}",f);
    println!("{}",b);
    println!("{}",decimal);
    println!("{}",hex);
    println!("{}",octal);
    println!("{}",binary);
    println!("{}",byte);
    println!("{}",name);
    println!("{}",tup.0);
    println!("{}",tup_1.1);
}
