fn parse_age(text: &str) -> Result<u32, std::num::ParseIntError> {
    let age = text.parse::<u32>()?;
    Ok(age)
}

fn main() {
    for input in ["44", "not-a-number"] {
        match parse_age(input) {
            Ok(age) => println!("age={age}"),
            Err(error) => println!("cannot parse {input:?}: {error}"),
        }
    }
}
