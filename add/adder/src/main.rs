use add_one;
use add_two;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus 1 is {}!",
        num,
        add_one::add_one(num)
    );

    let num_2 = 7;
    println!(
        "Hello, world! {} plus 2 is {}!",
        num_2,
        add_two::add_two(num_2)
    );
}
