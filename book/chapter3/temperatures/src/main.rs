use std::io;

fn main() {
    println!("Is your temperature in C or F?");
    let mut sign = String::new();

    io::stdin()
    .read_line(&mut sign)
    .expect("Failed to read input.");

    let sign: char = sign.trim().parse().expect("Please enter C or F.");

    println!("Enter the temperature you would like to convert.");
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read input.");

    let temperature: i16 = temperature.trim().parse().expect("Please enter a valid number.");
    
    let output: String = convert(temperature, sign);

    println!("{output}");
    
}

fn convert(temp: i16, s: char) -> String{
    if s == 'C' {
        let new_temp = temp * 9/5 + 32;
        let new_sign = 'F';
        return format!("{new_temp} {new_sign}")
    } else if s == 'F' {
        let new_temp = (temp - 32) * 5/9;
        let new_sign = 'C';
        return format!("{new_temp} {new_sign}")
    } else {
        return ("Invalid temperature, please enter C or F.").to_string()
    }

}