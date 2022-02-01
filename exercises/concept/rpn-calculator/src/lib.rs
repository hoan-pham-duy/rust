#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut isOperand = false;
    let mut result = None;
    let mut stack = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(value) => {
                stack.push(*value);
            }
            CalculatorInput::Add | CalculatorInput::Subtract | CalculatorInput::Multiply | CalculatorInput::Divide => {
                if stack.len() < 2 {
                    return None;
                }
                isOperand = true;
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match input {
                    CalculatorInput::Add => {stack.push(left + right)}
                    CalculatorInput::Subtract => {stack.push(left - right)}
                    CalculatorInput::Multiply => {stack.push(left * right)}
                    CalculatorInput::Divide => {stack.push(left/right)}
                    _ => ()
                };
            }
        }
    }
    if !isOperand && stack.len() != 1 {
        return None;
    }
    result = stack.pop();
    return result;
}
