mod operations;

fn main() {
    let addition = operations::Operation::Addition(5, 3);
    let subtraction = operations::Operation::Subtraction(10, 4);
    let multiplication = operations::Operation::Multiplication(2, 6);
    let division = operations::Operation::Division(12, 3);
    let division_by_zero = operations::Operation::Division(10, 0);

    println!("Addition: {}", calculate(addition).unwrap());
    println!("Subtraction: {}", calculate(subtraction).unwrap());
    println!("Multiplication: {}", calculate(multiplication).unwrap());
    println!("Division: {}", calculate(division).unwrap());

    match calculate(division_by_zero) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
