fn main() {
    for day in 1..13 {
      day_intro(day);
      for gift_day in (1..(day +1)).rev() {
        gift(
          gift_day,
          if gift_day == 1 && day != 1 {
            "and "
          } else {
            ""
          },
        );
      }
    }
}

fn day_intro(n :u32) {
  let day = match n {
    1 => "first",
    2 => "second",
    3 => "third",
    4 => "fourth",
    5 => "fifth",
    6 => "sixth",
    7 => "seventh",
    8 => "eighth",
    9 => "ninth",
    10 => "tenth",
    11 => "eleventh",
    12 => "twelth",
    _ => ""
  };

  println!(
    "On the {} day of Christmas, my true love gave to me: ", day
  );
}

fn gift(n: u32, and: &str) {
  let gift_text = match n {
      1 => "A Partridge in a Pear Tree",
      2 => "Two Turtle Doves",
      3 => "Three French Hens",
      4 => "Four Calling Birds",
      5 => "Five Golden Rings",
      6 => "Six Geese a Laying",
      7 => "Seven Swans a Swimming",
      8 => "Eight Maids a Milking",
      9 => "Nine Ladies Dancing",
      10 => "Ten Lords a Leaping",
      11 => "Eleven Pipers Piping",
      12 => "12 Drummers Drumming",
      _ => "",
  };

  println!("{}{}", and, gift_text);
}