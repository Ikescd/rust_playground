use std::io;

fn main() {
    // 1. l'utilisateur choisit vers quelle unité convertir sa témperature
    println!("Do you want to convert to Celsius or Fahrenheit? Input C or F");

    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Enter a value!");
    let unit: &str = unit.trim();
    let contrary_unit = if unit == "C" { "F" } else { "C" };

    // 2. l'utilisateur entre une température à convertir
    println!("Enter a value to convert in {unit}.");
    let mut value_to_convert = String::new();

    io::stdin()
        .read_line(&mut value_to_convert)
        .expect("Enter a valid value");

    // 3. nous retournons la température convertie
    let value_to_convert: f32 = value_to_convert.trim().parse().unwrap();
    let converted_value: f32 = match unit {
        "F" => (value_to_convert - 32.0) / 1.8,
        "C" => (value_to_convert * 1.8) + 32.0,
        _ => return,
    };
    println!("{value_to_convert}°{unit} is equal to {converted_value:.2}°{contrary_unit}")
}
