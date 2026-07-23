fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let multiplier = 3;
    let scale = |x: i32| x * multiplier;
    let result = {
        let base = add(2, 3);
        scale(base)
    };
    println!("result={result}");
}
