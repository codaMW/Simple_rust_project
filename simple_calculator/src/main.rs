mod tsado;
use std::io;

fn main() {
    
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter number: ");
    io::stdin().read_line(&mut num1).expect("error reading line");

    println!("Enter number: ");
    io::stdin().read_line(&mut num2).expect("error reading line");

    let num1: i32 = num1.trim().parse().expect("error reading value");
    let num2: i32 = num2.trim().parse().expect("error reading value");

    println!("{} + {} = {}", num1, num2, tsado::basic::add(num1,num2));
    println!("{} - {} = {}", num1, num2, tsado::basic::subtract(num1, num2));
    println!("{} x {} = {}", num1, num2, tsado::basic::multiply(num1, num2));
    tsado::basic::divide(num1, num2);

}
