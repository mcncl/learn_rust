use std::io;

fn main() {
  loop {
    println!("Fibonacci Numbers");

    println!("Enter a number to find out its positional Fibonacci value.");

    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Issue reading input");

    let input: u32 = match input.trim().parse() {
      Ok(num) => num,
      Err(_) => continue
    };

    println!("The value of Fibonacci position {} is {}", input, fibonacci(input))
  }
}

fn fibonacci(n: u32) -> u32 {
  match n {
      0 => 1,
      1 => 1,
      _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}