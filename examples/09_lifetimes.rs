fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let a = String::from("Flutter");
    let b = String::from("Rust");
    println!("longest={}", longest(&a, &b));
}
