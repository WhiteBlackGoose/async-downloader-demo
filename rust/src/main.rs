use std::sync::Arc;

use futures::{stream::FuturesUnordered, StreamExt};
use reqwest::header::CONNECTION;
use tokio::spawn;

async fn download(
    url: &'static str,
    mut count: u64,
    lim: u64,
) -> Vec<Result<String, reqwest::Error>> {
    let client = Arc::new(reqwest::Client::new());
    let mut res = vec![];
    let fut = || {
        let client = client.clone();
        async move {
            client
                .get(url)
                .header(CONNECTION, "close")
                .send()
                .await?
                .text()
                .await
        }
    };
    let mut set = FuturesUnordered::new();
    for _ in 0..lim.min(count) {
        set.push(spawn((fut)()));
        count -= 1;
    }
    while let Some(result) = set.next().await {
        res.push(result.unwrap());
        if count > 0 {
            set.push(spawn((fut)()));
            count -= 1;
        }
    }
    res
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let my_webpage = "http://127.0.0.1:8000/index.html";
    let count = 20000;
    let results = download(my_webpage, count, 300).await;
    for result in results {
        match result {
            Ok(r) => {
                assert!(r.len() == 6113 || r.len() == 25);
            }
            Err(e) => {
                eprintln!("Error: {:#?}", e);
            }
        }
    }
    println!("Done");
}
