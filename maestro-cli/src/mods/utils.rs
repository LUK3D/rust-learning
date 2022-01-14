use std::io;
/**Executar funcao */
pub fn run(){

  

    loop {
        println!("MAESTRO-> ");
        let cmd = input();
        println!("{}", cmd.trim()=="exit");

        if cmd.trim() == "exit" {
            println!("MAESTRO->Exiting...");
            break;
        }
    }
    
}


pub fn input() -> String{
    
    let mut cmd = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read input");

       return cmd;

}


pub fn witch