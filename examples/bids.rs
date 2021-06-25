use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let (bids, pending_items) = api.bids_get_options().slot_num(1).fetch().await.unwrap();

    println!("{:#?}, {}", bids, pending_items);
}
