use std::time::Duration;

async fn fetch_label(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(25)).await;
    format!("item-{id}")
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(fetch_label(1), fetch_label(2));
    println!("{a}, {b}");
}
