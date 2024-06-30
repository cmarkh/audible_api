use audible_api::{api::Client, auth::Auth};

#[tokio::main]
async fn main() {
    let auth = Auth::default("us").await.expect("Failed to sign in");
    let client = Client::new(auth).expect("Failed to create client");

    let library = client
        .get_library(None)
        .await
        .expect("Failed to get library");

    println!("{:#?}", library);
}
