use kiama::client::Client;

#[tokio::test]
async fn test_client() {
    let client = Client::from("youpi".to_string());
    let library = client.create_library("youpla".to_string());
    library.read();
}
