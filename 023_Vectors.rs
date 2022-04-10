fn main(){  

    let  mut my_vectors = vec![1,2,3,4,5];
    println!("{}" , my_vectors[1]);
    my_vectors.push(50);
    my_vectors.remove(0);

    for i in my_vectors.iter(){
    println!("{}" , i);
}
}