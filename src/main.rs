use std::io;

fn temp_ccalc(temp_f:i32) -> i32 {
    let temp_c = (temp_f - 32) * 5/9;
    return temp_c
}

fn temp_fcalc(temp_c:i32) -> i32 {
    let temp_f = (temp_c * 9/5) + 32;
    return temp_f
}

fn main() {
    println!("For Fahrenheit to Celsius press 'c', for Celsius to Fahrenheit press 'f'.");
    let mut calc = String::new();
    io::stdin()
        .read_line(&mut calc)
        .expect("failed to read from stdin.");

    match calc.as_str().trim() {
        "c" => {
            let mut input = String::new();
            println!("Type in the Temperature in Fahrenheit:");
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read from stdin.");
            let temp_f = input.trim();
            match temp_f.parse::<i32>() {
                Ok(i) => {
                    let temp_c = temp_ccalc(i);
                    println!("Temperature in Celsius: {}", temp_c)
                }
                Err(..) => println!("You did not enter a valid number. Your input: {}", temp_f)
            }
        },
        "f" => {
            let mut input = String::new();
            println!("Type in the Temperature in Celsius:");
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read from stdin.");
            let temp_c = input.trim();
            match temp_c.parse::<i32>() {
                Ok(i) => {
                    let temp_f = temp_fcalc(i);
                    println!("Temperature in Fahrenheit: {}", temp_f);
                }
                Err(..) => println!("You did not enter a valid value. Your input: {}", temp_c)
            }
        },
        _ => {
            println!("{} is an unknown value", calc.to_string());
        },
    }
}
