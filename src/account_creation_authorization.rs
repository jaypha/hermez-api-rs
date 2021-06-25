use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountCreationAuthorization {
    pub timestamp: String,
    pub hez_ethereum_address: String,
    pub bjj: String,
    pub signature: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostAccoutCreation<'a> {
    pub hez_ethereum_address: &'a str,
    pub bjj: &'a str,
    pub signature: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Success {
    pub success: String,
}

mod test {
    #[test]
    fn test_json() {
        let json = r#"{
              "hezEthereumAddress": "hez:0x74a549b410d01d9eC56346aE52b8550515B283b2",
              "bjj": "hez:dEZ-Tj7d5h0TAqbnRTTYURYDEo5KZzB87_2WknUU8gCN",
              "signature": "0x8db6db2ad6cbe21297fb8ee01c59b01b52d4df7ea92a0f0dee0be0075a8f224a06b367407c8f402cfe0490c142a1c92da3fc29b51162ae160d35e1577d3071bb01",
              "timestamp": "2020-11-17T13:25:36.784295Z"
            }"#;

        let _structure: super::AccountCreationAuthorization = serde_json::from_str(json).unwrap();
    }
}
