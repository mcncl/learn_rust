use std::io;

fn main() {
  loop {
    println!("Are we converting from °C or °F? Input 'c' or 'f'.");

    let mut unit = String::new();

    io::stdin()
      .read_line(&mut unit)
      .expect("Failed to read input.");

    let chosen_unit;

    if unit.trim().to_lowercase() == "c" {
      chosen_unit = "Celsius";

    } else if unit.trim().to_lowercase() == "f" {
      chosen_unit = "Fahrenheit";
    } else {
      println!("Please input 'c' or 'f' only");
      continue
    }

    println!("Input a temperature in {}.", chosen_unit);

    let mut chosen_temp = String::new();

    io::stdin()
      .read_line(&mut chosen_temp)
      .expect("Failed to read input.");

    let chosen_temp: f64 = match chosen_temp.trim().parse() {
      Ok(num) => num,
      Err(_) => continue
    };

    if unit.trim().to_lowercase() == "c" {
      println!("{}°C is {:.1}°F", chosen_temp, convert_c(chosen_temp))
    } else if unit.trim().to_lowercase() == "f" {
      println!("{}°F is {:.1}°C", chosen_temp, convert_f(chosen_temp))
    } else {continue}
  }
}

fn convert_c(t: f64) -> f64 {
  (t * 9.0/5.0) + 32.0
}

fn convert_f(t: f64) -> f64 {
  (t - 32.0) * 5.0/9.0
}