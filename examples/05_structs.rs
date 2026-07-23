#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: impl Into<String>, age: u32) -> Self {
        Self { name: name.into(), age }
    }

    fn birthday(&mut self) {
        self.age += 1;
    }

    fn label(&self) -> String {
        format!("{} ({})", self.name, self.age)
    }
}

fn main() {
    let mut user = User::new("Ada", 36);
    user.birthday();
    println!("{}", user.label());
}
