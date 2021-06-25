use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let (accounts, pending_items) = api.accounts_get_options().fetch().await.unwrap();

    println!("{:#?}, {}", accounts, pending_items);

    let (accounts, pending_items) = api.accounts_get_options().fetch().await.unwrap();

    println!("{:#?}, {}", accounts, pending_items);

    let account = api.get_account("hez:DAI:4444").await.unwrap();
    println!("{:#?}", account);
}
