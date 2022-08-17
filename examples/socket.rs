use dns_sd2::something;

#[tokio::main]
pub async fn main() {
    something().await.unwrap();
}
