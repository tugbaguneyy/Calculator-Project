use std::io::{self, Write};
fn main() {
    println!("Hello! Please enter a operation number:(1.Add 2.Divide 3.Multiply 4.Substact)");
    let mut op_input = String::new(); //kullanıcıdan işlemi almak için
    io::stdin().read_line(&mut op_input);
    std::io::stdout().flush();//ekranı güncellemek için
    
    println!("Enter the first number");//kullanıcıdan birinci sayıyı almak için
    let mut num1_input = String::new(); 
    io::stdin().read_line(&mut num1_input);
    std::io::stdout().flush();//ekranı güncellemek için
    
    println!("Enter the second number");//kullanıcıdan ikinci sayıyı almak için
    let mut num2_input = String::new(); 
    io::stdin().read_line(&mut num2_input);
    std::io::stdout().flush();//ekranı güncellemek için

    //convert işlem
    let op: i16 = op_input.trim().parse().expect("Invalid number");
    let num1: f64 = num1_input.trim().parse().expect("Invalid number");
    let num2: f64 = num2_input.trim().parse().expect("Invalid number");
    println!("{} {}",&num1,&num2);

    //girilen işleme göre operation oluşturma
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