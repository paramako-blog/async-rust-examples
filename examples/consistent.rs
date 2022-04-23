use std::time::Duration;

use tokio;

#[tokio::main]
async fn main() {
    task("Long Task 1", true).await;
    task("Short Task 1", false).await;
    task("Long Task 2", true).await;
    task("Short Task 2", false).await;
}

async fn task(name: &str, long: bool) {
    println!("Executing {}", name);

    if long {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("{} executed", name);
}
