use std::io;

fn main() {
    println!("A simple Rust calculator");

    let mut op = String::new();    
    println!("Choose: +, - / or * (Enter quit to exit):");

    loop {   
        // Clear op variable each iteration
        op.clear();

        // Take user input to choose operator
        io::stdin()
            .read_line(&mut op)
            .expect("Could not read line");
    
        let op = op.trim();
        
        // Quit app
        if op == "quit" {
            println!("Terminating calculator");
            break;
        }
        
        if op != "+" && op != "-" && op != "/" && op != "*" {
            println!("Not a valid operator!\nChoose operator:");            
            continue;
        } 
        
        println!("You chose: {} \nEnter first number:", op);

        // Enter first number, trim and parse --> str --> i32
        let mut num_a= String::new();
        io::stdin()
            .read_line(&mut num_a)
            .expect("Not a number!");

        let num_a: i32 = num_a.trim().parse().expect("Please type a number");

        // Enter second number, trim and parse --> str --> i32
        println!("Enter second number:");
        
        let mut num_b = String::new();
        io::stdin()
            .read_line(&mut num_b)
            .expect("Not a number!");

        let num_b: i32 = num_b.trim().parse().expect("Please type a number");

        if op == "+" {
            let sum: i32 = num_a + num_b;            
            println!("{} + {} = {}", num_a, num_b, sum);

            println!("Do you want to continue (y/n)?")
        }

    }

}
