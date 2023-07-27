use std::io::{self, Write};

fn main() {
    println!("Hello! Please enter an operation number: (1.Add 2.Divide 3.Multiply 4.Substract)");

    let op: i16 = loop {
        let mut op_input = String::new();
        io::stdin().read_line(&mut op_input).expect("Failed to read input");

        // parse() ile dönüşüm denemek
        match op_input.trim().parse::<i16>() {
            Ok(num) => break num, // Geçerli bir sayı dönüştüyse döngüyü sonlandır
            Err(_) => println!("Invalid operation number. Please try again."),
        };
    };

    println!("Enter the first number:");
    let num1: f64 = loop {
        let mut num1_input = String::new();
        io::stdin().read_line(&mut num1_input).expect("Failed to read input");

        // parse() ile dönüşüm denemek
        match num1_input.trim().parse::<f64>() {
            Ok(num) => break num, // Geçerli bir sayı dönüştüyse döngüyü sonlandır
            Err(_) => println!("Invalid number. Please try again."),
        };
    };

    println!("Enter the second number:");
    let num2: f64 = loop {
        let mut num2_input = String::new();
        io::stdin().read_line(&mut num2_input).expect("Failed to read input");

        // parse() ile dönüşüm denemek
        match num2_input.trim().parse::<f64>() {
            Ok(num) => break num, // Geçerli bir sayı dönüştüyse döngüyü sonlandır
            Err(_) => println!("Invalid number. Please try again."),
        };
    };

    // Girilen işleme göre operation oluşturma
    let result = match op {
        1 => Operation::Add(num1, num2),
        2 => Operation::Divide(num1, num2),
        3 => Operation::Multiply(num1, num2),
        4 => Operation::Substract(num1, num2),
        _ => panic!("Invalid operation selection."),
    };
    println!("Sonuç: {}", result.calculate());
}

enum Operation {
    Add(f64, f64),
    Substract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add(x, y) => x + y,
            Operation::Substract(x, y) => x - y,
            Operation::Multiply(x, y) => x * y,
            Operation::Divide(x, y) => x / y,
        }
    }
}
