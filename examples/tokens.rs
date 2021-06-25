use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let (tokens, pending_items) = api.tokens_get_options().fetch().await.unwrap();

    println!("{:#?}, {}", tokens, pending_items);

    let (tokens, pending_items) = api.tokens_get_options().ids(&[2, 3]).fetch().await.unwrap();

    println!("{:#?}, {}", tokens, pending_items);

    let (tokens, pending_items) = api
        .tokens_get_options()
        .symbols(&["ETH", "HEZ", "LINK"])
        .limit(2)
        .fetch()
        .await
        .unwrap();

    println!("{:#?}, {}", tokens, pending_items);

    let token = api.get_token(2).await.unwrap();
    println!("{:#?}", token);
}
