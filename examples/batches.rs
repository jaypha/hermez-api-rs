use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let (batches, pending_items) = api.batches_get_options().fetch().await.unwrap();

    println!("{:#?}, {}", batches, pending_items);

    let (batches, pending_items) = api.batches_get_options().fetch().await.unwrap();

    println!("{:#?}, {}", batches, pending_items);

    let batch = api.get_batch(18).await.unwrap();
    println!("{:#?}", batch);

    let batch = api.get_full_batch(18).await.unwrap();
    println!("{:#?}", batch);
}
