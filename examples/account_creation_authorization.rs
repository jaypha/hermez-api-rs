use hermez_api::HermezApi;

#[async_std::main]
async fn main() {
    let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

    let info = api
        .get_account_creation_authorization("hez:0xaa942cfcd25ad4d90a62358b0dd84f33b398262a")
        .await
        .unwrap();

    println!("{:#?}", info);
}
