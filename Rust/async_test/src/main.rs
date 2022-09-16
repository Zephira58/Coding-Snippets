use tokio::spawn;

use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            async_test(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn async_test(i: i32) {
    println!("[{i}]I'm an async function!");
    let s1 = read_from_database().await;
    println!("[{i}]First read: {}", s1);
    let s2 = read_from_database().await;
    println!("[{i}]Second read: {}", s2);
}

async fn read_from_database() -> String {
    sleep(Duration::from_secs(1)).await;
    "DB result".to_owned()
}