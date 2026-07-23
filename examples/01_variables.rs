fn main() {
    let immutable = 5;
    let mut mutable = 10;
    mutable += 10;

    let text = "42";
    let text: i32 = text.parse().expect("valid integer");

    println!("immutable={immutable}, mutable={mutable}, shadowed={text}");
}
