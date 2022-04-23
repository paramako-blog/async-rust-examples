use std::time::Duration;

use tokio;

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();

    futures.push(task("Long Task 1", true));
    futures.push(task("Short Task 1", false));
    futures.push(task("Long Task 2", true));
    futures.push(task("Short Task 2", false));

    futures::future::join_all(futures).await;
}

async fn task(name: &str, long: bool) {
    println!("Executing {}", name);

    if long {
        std::thread::sleep(Duration::from_secs(1));
    }

    println!("{} executed", name);
}
