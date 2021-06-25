use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let (transactions_history, pending_items) = api
        .transactions_history_get_options()
        .fetch()
        .await
        .unwrap();

    println!("{:#?}, {}", transactions_history, pending_items);
}
