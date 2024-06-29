use audible::auth::Auth;

#[tokio::main]
async fn main() {
    let country_code = "us";
    let auth = Auth::default(country_code).await.unwrap();
    dbg!(auth);
}
