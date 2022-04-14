use std::env::{args, Args};

fn main() {
    println!("1) Hello, world!");

    // Fetching command line arguments
    let mut args: Args = args();
    println!("2) {:?}", args);

    // Accessing each argument
    let zero = args.nth(0);
    // if it exists we can unwrap() it.
    println!("3) {:?}", zero);

    let first = args.nth(0).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();
    println!("4) {:?} {} {}", first, operator, second);

    let first_num: f32 = first.parse().unwrap(); // normal method to specify type
    let second_num = second.parse::<f32>().unwrap(); // turbo fish method to specify type
    let result = operate(operator, first_num, second_num);
    println!("5) Result: {:?}", result);
    
    println!("6) {:?}", output(first_num, operator, second_num, result));

    let result2 = operate_using_match(operator, first_num, second_num);
    println!("7) {:?}", output(first_num, operator, second_num, result2));
}


fn operate(operator: char, first_number: f32, second_number: f32) -> f32{
    if operator == '+' {
        return first_number + second_number
    } else if operator == '-' {
        return first_number - second_number
    } else if operator == '*' {
        return first_number * second_number
    } else if operator == '/' {
        return first_number/second_number;
    } else{
        println!("hello");
        return 0.0;
    }
}

fn operate_using_match(operator: char, first_number: f32, second_number: f32) -> f32{
    match operator{
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator used!!")
    }
}

fn output(first_number: f32, operator: char ,second_number: f32, result: f32) -> String{
    return format!("{} {} {} = {}", first_number, operator, second_number, result);
}