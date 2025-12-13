mod dot_not;

fn main() {
    
    let person = dot_not::dot_notation("Yankho", 23);

    println!("{} is {} years old", person.0, person.1);
}
