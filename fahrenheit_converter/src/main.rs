use std::io;
mod temp_conv;

fn main() {
    let mut temp = String::new();

    println!("Enter the temperature in °C: ");
    io::stdin().read_line(&mut temp).expect("failure to read value");

    let temp:f32 = temp.trim().parse().expect("failed to read value");

    println!("{:.1}°C = {:.1} °F", temp, temp_conv::fah_to_cels(temp)); 
}
