use audible::{api::Client, auth::Auth};

#[tokio::main]
async fn main() {
    let auth = Auth::default("us").await.expect("Failed to sign in");
    let client = Client::new(auth).expect("Failed to create client");

    let library = client.get_library().await.expect("Failed to get library");
    println!("{:#?}", library);
}
