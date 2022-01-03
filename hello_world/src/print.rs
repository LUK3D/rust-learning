pub fn run(){
    //Print a line to console
    println!("Hello from print.rs file");

    //Basic Formatting 
    println!("Numero: {}",1);

    //Positional Args
    println!("{0} is from {1} and {0} likes to {2}","Lukebana","Angola","code");

    //Named Args
    println!("{name} likes to play {activity}",name="Filipe", activity ="Video Games");

}   