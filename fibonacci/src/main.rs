fn main() {
  println!("Fibonacci Numbers");

  println!("Enter a number to find out its positional Fibonacci value.")
}

fn fibonacci(n: u32) -> u32 {
  match n {
      0 => 1,
      1 => 1,
      _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}
