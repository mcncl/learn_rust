#[derive(Debug)]

struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn calculate_area(&self) -> u32{
    self.width * self.height
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size
    }
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 17,
    height: 10
  };

  let square = Rectangle::square(4);

  println!("The dimensions of the rect are {:?}", rect1);

  println!("The area of the rect is {}", rect1.calculate_area());

  println!("The dimensions of the square are {:?}", square);

  println!("the area of the square is {:?}", square.calculate_area())
}