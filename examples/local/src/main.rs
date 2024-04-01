use kiama::client::Client;
use std::env;

#[tokio::main]
async fn main() {
    let mut dir = env::temp_dir();
    dir.push("kiama-test");
    let dir_path = dir.to_str().unwrap();

    let client = Client::from(dir_path.to_string());
    let library = client.create_library("toto".to_string()).await;
    library.read().await;
}
