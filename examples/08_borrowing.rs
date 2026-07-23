fn length(text: &str) -> usize {
    text.len()
}

fn append_bang(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut text = String::from("hello");
    let len = length(&text);
    println!("before: {text}, bytes={len}");

    append_bang(&mut text);
    println!("after: {text}");
}
