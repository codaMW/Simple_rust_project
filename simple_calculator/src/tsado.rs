pub mod basic {

    pub fn add(a: i32, b: i32) -> i32 {

        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {

        a - b
    }
    pub fn multiply(a: i32, b: i32) -> i32 {

        a * b
    }

    pub fn divide(a: i32, b: i32)  {

        if b == 0 {

            println!("Invalid mathematical operation!!");
            
        } else {

            println!("{} ÷ {} = {:.2}", a, b, a as f32 / b as f32);
        }
    }
}

