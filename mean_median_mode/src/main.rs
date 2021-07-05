use std::collections::HashMap;

fn main() {
    let mut v = vec![4, 11, 3, 32, 5, 54, 3, 8, 9];
    let mut sum = 0;

    // Loop through values in V, add their value to SUM
    for i in &v {
        sum += i;
    }

    println!("Calculating mean, median & mode for {:?}\n", v);
    mean(sum as f64, v.len() as f64);
    println!("The median value is {}", median(&mut v));
    println!("The mode value is {:?}", mode(&mut v));
}

fn mean(n: f64, l: f64) {
    println!("The mean value is {}", f64::from(n / l));
}

fn median(a: &mut [u32]) -> u32 {
    a.sort();
    let mid = a.len() / 2;
    a[mid]
}

fn mode(vec: &[u32]) -> Option<u32> {
    let mut counts = HashMap::new();
    vec.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}
