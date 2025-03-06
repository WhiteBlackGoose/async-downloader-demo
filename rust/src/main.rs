use futures::{stream::FuturesUnordered, StreamExt};
use reqwest::header::CONNECTION;

async fn download(url: &str, mut count: u64, lim: u64) -> Vec<Result<String, reqwest::Error>> {
    let client = reqwest::Client::new();
    let mut res = vec![];
    let fut = || async {
        client
            .get(url)
            .header(CONNECTION, "close")
            .send()
            .await?
            .text()
            .await
    };
    let mut set = FuturesUnordered::new();
    for _ in 0..lim.min(count) {
        set.push((fut)());
        count -= 1;
    }
    while let Some(result) = set.next().await {
        res.push(result);
        if count > 0 {
            set.push((fut)());
            count -= 1;
        }
    }
    res
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let my_webpage = "http://127.0.0.1:8000/index.html";
    let count = 2000;
    let results = download(my_webpage, count, 300).await;
    for result in results {
        match result {
            Ok(r) => {
                assert_eq!(r.len(), 25);
            }
            Err(e) => {
                eprintln!("Error: {:#?}", e);
            }
        }
    }
    println!("Done");
}
