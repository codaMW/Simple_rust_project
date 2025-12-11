mod max_num;
use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();

    println!("Enter 1st number: ");
    io::stdin().read_line(&mut num1).expect("error reading value");

    println!("Enter 2nd number: ");
    io::stdin().read_line(&mut num2).expect("error reading value");

    println!("Enter 3rd number: ");
    io::stdin().read_line(&mut num3).expect("error reading value");

    let num1: i32 = num1.trim().parse().expect("error");
    let num2: i32 = num2.trim().parse().expect("error");
    let num3: i32 = num3.trim().parse().expect("error");

    let max_number: i32 = max_num::cmp_num(num1, num2, num3);

    if num1 == num2 && num2 == num3 {
        println!("All numbers are equal");
    } else{
        println!("Max number between the 3 is {}", max_number);
    }

}
