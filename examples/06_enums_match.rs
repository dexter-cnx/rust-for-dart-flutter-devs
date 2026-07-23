#[derive(Debug)]
enum LoadState {
    Idle,
    Loading,
    Success { items: Vec<String> },
    Failure(String),
}

fn render(state: &LoadState) -> String {
    match state {
        LoadState::Idle => "Idle".to_string(),
        LoadState::Loading => "Loading...".to_string(),
        LoadState::Success { items } => format!("{} items", items.len()),
        LoadState::Failure(message) => format!("Error: {message}"),
    }
}

fn main() {
    let states = [
        LoadState::Idle,
        LoadState::Loading,
        LoadState::Success { items: vec!["A".into(), "B".into()] },
        LoadState::Failure("Network unavailable".into()),
    ];
    for state in &states {
        println!("{}", render(state));
    }
}
