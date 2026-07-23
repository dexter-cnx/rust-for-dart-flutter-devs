fn main() {
    let values = vec![1, 2, 3, 4, 5, 6];
    let squares: Vec<i32> = values
        .iter()
        .filter(|value| **value % 2 == 0)
        .map(|value| value * value)
        .collect();

    println!("values={values:?}");
    println!("even squares={squares:?}");
}
