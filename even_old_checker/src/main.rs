use std::io;

fn even_old_check(a: i32) {

    if a % 2 == 0 {
        println!("{a} is Even");
    } else {
        println!("{a} is Odd");
    }
}

fn main() {

    let mut num = String::new();

    println!("Enter number: ");
    io::stdin().read_line(&mut num).expect("error reading value");

    let num: i32 = num.trim().parse().expect("error reading value");
    
    even_old_check(num);
}
