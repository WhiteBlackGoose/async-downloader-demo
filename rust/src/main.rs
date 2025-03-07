use std::sync::Arc;

use futures::future::join_all;
use reqwest::header::CONNECTION;
use tokio::{spawn, sync::Semaphore};

async fn download(url: &'static str, count: u64, lim: u64) -> Vec<Result<String, reqwest::Error>> {
    let client = Arc::new(reqwest::Client::new());
    let sem = Arc::new(Semaphore::new(lim as usize));
    join_all((0..count).map(|_| {
        spawn({
            let client = client.clone();
            let sem = sem.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();
                client
                    .get(url)
                    .header(CONNECTION, "close")
                    .send()
                    .await?
                    .text()
                    .await
            }
        })
    }))
    .await
    .into_iter()
    .map(|r| r.unwrap())
    .collect()
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let my_webpage = "http://127.0.0.1:8000/index.html";
    let count = 200000;
    let results = download(my_webpage, count, 300).await;
    for result in results {
        match result {
            Ok(r) => {
                assert!(r.len() == 6113 || r.len() == 25);
            }
            Err(e) => {
                panic!("Error: {:#?}", e);
            }
        }
    }
    println!("Done");
}
