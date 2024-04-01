use kiama::client::Client;
use polars::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    let mut dir = env::temp_dir();
    dir.push("kiama-test");
    let dir_path = dir.to_str().unwrap();

    let client = Client::from(dir_path.to_string());
    let library = client.create_library("toto".to_string()).await;

    let q = LazyCsvReader::new("data/date_series-AAPL-1day.csv")
        .has_header(true)
        .with_separator(b';')
        .finish()
        .expect("data");
    let df = q.collect().expect("data");
    println!("{}", df);

    let df2 = library.read().await;
    println!("{}", df2);
}
