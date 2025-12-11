use std::io;

fn main() {
    let mut subject_score = String::new();

    println!("Enter subject score: ");
    io::stdin()
        .read_line(&mut subject_score)
        .expect("error reading value");

    let subject_score: u32 = subject_score.trim().parse().expect("error");

    let gpa = match subject_score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => 'X',
    };

    println!("{} = {}", subject_score, gpa);
}
