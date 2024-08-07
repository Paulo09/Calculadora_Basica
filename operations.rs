enum Operation {
    Addition(i32, i32),
    Subtraction(i32, i32),
    Multiplication(i32, i32),
    Division(i32, i32),
}

fn calculate(operation: Operation) -> Result<i32, &'static str> {
    match operation {
        Operation::Addition(a, b) => Ok(a + b),
        Operation::Subtraction(a, b) => Ok(a - b),
        Operation::Multiplication(a, b) => Ok(a * b),
        Operation::Division(a, b) => {
            if b == 0 {
                Err("Cannot divide by zero")
            } else {
                Ok(a / b)
            }
        }
    }
}
