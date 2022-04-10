enum Dierection{
    UP ,
    DOWN ,
    LEFT ,
    RIGHT ,

}

fn main(){

    let player_dierction:Dierection = Dierection::UP;
        match player_dierction {

            Dierection::UP => println!("We are heading up!"),
            Dierection::DOWN => println!("We are heading down!"),
            Dierection::LEFT => println!("We are heading left!"),
            Dierection::RIGHT => println!("We are heading right!"),
        }
}
