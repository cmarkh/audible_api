use audible::auth::Auth;

#[tokio::main]
async fn main() {
    let auth = Auth::default("us").await.expect("Failed to sign in");
    dbg!(auth);
}
