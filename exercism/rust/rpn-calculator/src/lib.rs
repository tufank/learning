#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut calc: Vec<i32> = vec![];
    let mut result: Option<i32> = None;

    for input in inputs {
        result = match input {
            CalculatorInput::Value(x) => Some(*x),
            CalculatorInput::Add => {
                if calc.len() > 1 {
                    let y = calc.pop().unwrap();
                    let x = calc.pop().unwrap();
                    Some(x + y)
                } else {
                    None
                }
            }
            CalculatorInput::Subtract => {
                if calc.len() > 1 {
                    let y = calc.pop().unwrap();
                    let x = calc.pop().unwrap();
                    Some(x - y)
                } else {
                    None
                }
            }
            CalculatorInput::Multiply => {
                if calc.len() > 1 {
                    let y = calc.pop().unwrap();
                    let x = calc.pop().unwrap();
                    Some(x * y)
                } else {
                    None
                }
            }
            CalculatorInput::Divide => {
                if calc.len() > 1 {
                    let y = calc.pop().unwrap();
                    let x = calc.pop().unwrap();
                    Some(x / y)
                } else {
                    None
                }
            }
        };

        match result {
            Some(value) => calc.push(value),
            None => break,
        }
    }

    if calc.len() != 1 || result == None {
        None
    } else {
        Some(calc.pop().unwrap())
    }
}
