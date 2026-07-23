fn consume(text: String) {
    println!("consumed: {text}");
}

fn main() {
    let original = String::from("hello");
    let cloned = original.clone();

    consume(original);
    // original is no longer available here.

    println!("cloned remains: {cloned}");

    let x = 42;
    let y = x; // i32 implements Copy.
    println!("x={x}, y={y}");
}
