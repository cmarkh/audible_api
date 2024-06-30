use audible::auth::browser_signin::serve_signin;

#[tokio::main]
async fn main() {
    let auth = serve_signin().await.expect("Failed to sign in");
    dbg!(auth);
}
