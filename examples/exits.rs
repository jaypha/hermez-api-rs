use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let (exits, pending_items) = api.exits_get_options().fetch().await.unwrap();

    println!("{:#?}, {}", exits, pending_items);

    let (exits, pending_items) = api.exits_get_options().token_id(2).fetch().await.unwrap();

    println!("{:#?}, {}", exits, pending_items);

    let (exits, pending_items) = api
        .exits_get_options()
        .account_index("hez:ETH:256")
        .only_pending_withdraws(true)
        .fetch()
        .await
        .unwrap();

    println!("{:#?}, {}", exits, pending_items);

    let exit = api.get_exit(5, "hez:ETH:256").await.unwrap();
    println!("{:#?}", exit);
}
