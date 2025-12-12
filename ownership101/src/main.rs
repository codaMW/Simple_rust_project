fn main() {
    let mut s1 = String::from("Yankho");

    s1.push_str(" Ngolleka"); //s1 own the value "Yankho Ngolleka" from this scope downwards

    println!("Hello, {}", s1);

    let s2 = s1; //from this point onwards s1 has lost ownership to the value. ownership has "MOVE"
                 //d to s2

    println!("Hi {}", s2);

    //if we try to access s1 from this point it will throw an error since it no longer has access
    //to the value.
    
    println!("Error {}", s1);
}
