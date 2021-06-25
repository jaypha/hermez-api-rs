use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let (slots, pending_items) = api
        .slots_get_options()
        .max_slot_num(1000)
        .fetch()
        .await
        .unwrap();

    println!("{:#?}, {}", slots, pending_items);

    let mut options = api.slots_get_options();

    options.min_slot_num(10).max_slot_num(500);

    let (slots, pending_items) = options.fetch().await.unwrap();

    println!("{:#?}, {}", slots, pending_items);

    let exit = api.get_slot(5).await.unwrap();
    println!("{:#?}", exit);
}
