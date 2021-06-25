use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let (coordinators, pending_items) = api.coordinators_get_options().fetch().await.unwrap();

    println!("{:#?}, {}", coordinators, pending_items);
}
