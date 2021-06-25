use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    let health = api.get_health().await.unwrap();

    println!("status: {}", health.status);
    println!("version: {}", health.version);

    println!("{:#?}", health);
}
