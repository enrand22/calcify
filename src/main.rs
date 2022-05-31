use std::io::stdin;

use regex::Regex;

fn main() {
    println!("Hello, world!");

    println!("Por favor introduce tu expresion: ");
    let mut expression = String::new();
    stdin().read_line(&mut expression).unwrap();
 
    expression = apply_multiply(expression);
    expression = apply_sum(expression);
    
    println!("Resultado: {}", expression)
}

fn apply_sum(expression: String) -> String {
    apply_operator(expression, "+")
}

fn apply_multiply(expression: String) -> String {
    apply_operator(expression, "*")
}

fn apply_operator(mut expression: String, operator: &str) -> String {
    let left_operand: &str = r"(\d+)\s?\";
    let right_operand: &str = r"\s?(\d+)";
    let rx = format!(r"{}{}{}",left_operand,operator,right_operand);
    let rx: Regex = Regex::new(&rx).unwrap();

    loop {
        let caps = rx.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_exp = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        
        let result = match operator {
            "+" => left_value + right_value,
            "*" => left_value * right_value,
             _  => 0 
        };

        expression = expression.replace(cap_exp, &result.to_string());
    }
    expression
}