fn display_name(name: Option<&str>) -> String {
    name.map(str::to_uppercase)
        .unwrap_or_else(|| "GUEST".to_string())
}

fn main() {
    println!("{}", display_name(Some("Ada")));
    println!("{}", display_name(None));
}
