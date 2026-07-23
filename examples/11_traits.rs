trait Describe {
    fn describe(&self) -> String;
}

struct Movie {
    title: String,
    rating: f32,
}

impl Describe for Movie {
    fn describe(&self) -> String {
        format!("{} — {:.1}/10", self.title, self.rating)
    }
}

fn print_description(value: &impl Describe) {
    println!("{}", value.describe());
}

fn main() {
    let movie = Movie { title: "Ferris Crab's Day Off".into(), rating: 9.0 };
    print_description(&movie);
}
