fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    let literal: &str = "Flutter developer";
    let owned = String::from(literal);
    println!("{}", greet(&owned));
    println!("chars={}", "สวัสดี".chars().count());
}
