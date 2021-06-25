use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    let config = api.get_config().await.unwrap();

    println!("{:#?}", config);
}
