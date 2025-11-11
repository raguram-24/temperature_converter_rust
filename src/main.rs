use std::io;
use std::str:: FromStr;
use std::fmt::Debug;
fn main() {
    let mut flag: bool = false;
    while ! flag {
        println!("Select the Temperature unit to convert");
        println!("1. C for Fahrenheit to Celsius");
        println!("2. F for Celsius to Fahrenheit");
        println!("3. X for Exit the Program");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read the Choice");
        let user_input_char : char = user_input.chars().nth(0).unwrap();
        if user_input_char == 'C' || user_input_char == 'c'  {
            println!("Enter the Temperature in Fahrenheit");
            let mut input_temp = String::new();
            io::stdin().read_line(&mut input_temp).expect("Failed to read the Temperature");
            let temp : f64 = input_temp.trim().parse().expect("Failed to change the String to Float");
            let result : f64 = fahrenheit_to_celsius(temp);
            println!("Temperature in Celsius : {}", result);
            println!("=======================================================================");
        }
        else if user_input_char == 'F' || user_input_char == 'f'  {
            println!("Enter the Temperature in Celsius");
            let mut input_temp = String::new();
            io::stdin().read_line(&mut input_temp).expect("Failed to read the Temperature");
            let temp : f64 = input_temp.trim().parse().expect("Failed to change the String to Float");
            let result : f64 = celsius_to_fahrenheit(temp);
            println!("Temperature in Fahrenheit : {}", result);
            println!("=======================================================================");
        }
        else if user_input_char == 'x' || user_input_char == 'x'  {
            flag = true;
            println!("Exiting Program");
            if flag {
                break;
            }
        }else {
            println!("Please enter a valid Option")
        }
    }
}
// Function for converting the Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0)/(9.0/5.0)
}

// Function for converting the Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0/5.0 + 32.0
}


