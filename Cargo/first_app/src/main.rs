use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Informe um valor");

    match io::stdin().read_line(&mut input){
        Ok(_)=>{
            print_numbers(14,45);
        },
        Err(e)=>{

        }
    }
   
}

fn print_numbers(n1:u64, n2:u64){
    if n1<10{
        println!("Numero muito baixo");
        print_numbers(n1,n2)
    }else{
        println!("A soma de  {}  e {} é:", n1,n2);

    }
}
