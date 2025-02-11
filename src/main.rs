use std::io;

fn main() {
    println!("A simple Rust calculator");

    let mut op = String::new();    
    println!("Choose: +, - / or * (Enter quit to exit):");

    loop {        
        io::stdin()
            .read_line(&mut op)
            .expect("Could not read line");
    
        let op = op.trim();
    
        if op == "quit" {
            println!("Terminating calculator");
            break;
        }

        if op != "+" && op != "-" && op != "/" && op != "*" {
            println!("Not a valid operator!");
            return;
        } 
        println!("You chose: {}", op);
    }

}
