use super::http::Http;
use serde::{Deserialize, Serialize};
use url::Url;

use super::ErrorKind;
use super::PaginationOrder;

use super::tokens::Token;

pub struct TransactionsHistoryGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,
    token_id: Option<u32>,
    hez_ethereum_address: Option<&'a str>,
    from_hez_ethereum_address: Option<&'a str>,
    to_hez_ethereum_address: Option<&'a str>,
    bjj: Option<&'a str>,
    from_bjj: Option<&'a str>,
    to_bjj: Option<&'a str>,
    account_index: Option<&'a str>,
    from_account_index: Option<&'a str>,
    to_account_index: Option<&'a str>,
    batch_num: Option<u32>,
    r#type: Option<TransactionType>,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> TransactionsHistoryGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,
            token_id: None,
            hez_ethereum_address: None,
            from_hez_ethereum_address: None,
            to_hez_ethereum_address: None,
            bjj: None,
            from_bjj: None,
            to_bjj: None,
            account_index: None,
            from_account_index: None,
            to_account_index: None,
            batch_num: None,
            r#type: None,

            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(token_id, u32);
    setter!(hez_ethereum_address, &'a str);
    setter!(from_hez_ethereum_address, &'a str);
    setter!(to_hez_ethereum_address, &'a str);
    setter!(bjj, &'a str);
    setter!(from_bjj, &'a str);
    setter!(to_bjj, &'a str);
    setter!(account_index, &'a str);
    setter!(from_account_index, &'a str);
    setter!(to_account_index, &'a str);
    setter!(batch_num, u32);
    setter!(r#type, TransactionType);

    pagination_setters!();

    pub async fn fetch(&self) -> Result<(Vec<HistoryTransaction>, u64), super::ErrorKind> {
        if is_more_than_one_defined!(
            self.hez_ethereum_address,
            self.from_hez_ethereum_address,
            self.to_hez_ethereum_address,
            self.bjj,
            self.from_bjj,
            self.to_bjj,
            self.account_index,
            self.from_account_index,
            self.to_account_index
        ) {
            return Err(ErrorKind::Api(String::from(
                "Get Transactions History: hez_ethereum address, from_hez_ethereum address, to_hez_ethereum address, bjj, from_bjj, to_bjj, account_index, from_account_index and to_account_index are incompatible with each other",
            )));
        }

        if is_more_than_one_defined!(
            self.token_id,
            self.account_index,
            self.from_account_index,
            self.to_account_index
        ) {
            return Err(ErrorKind::Api(String::from(
                "Get Transactions History: token_id, account_index, from_account_index and to_account_index are incompatible with each other",
            )));
        }

        let mut url = self.url.join("transactions-history").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            fetch_stmt!(self, query_pairs, token_id, "tokenId");
            fetch_stmt!(
                self,
                query_pairs,
                hez_ethereum_address,
                "hezEthereumAddress",
                &str
            );
            fetch_stmt!(
                self,
                query_pairs,
                from_hez_ethereum_address,
                "fromHezEthereumAddress",
                &str
            );
            fetch_stmt!(
                self,
                query_pairs,
                to_hez_ethereum_address,
                "toHezEthereumAddress",
                &str
            );
            fetch_stmt!(self, query_pairs, bjj, "BJJ", &str);
            fetch_stmt!(self, query_pairs, from_bjj, "fromBJJ", &str);
            fetch_stmt!(self, query_pairs, to_bjj, "toBJJ", &str);
            fetch_stmt!(self, query_pairs, account_index, "accountIndex", &str);
            fetch_stmt!(
                self,
                query_pairs,
                from_account_index,
                "fromAccountIndex",
                &str
            );
            fetch_stmt!(self, query_pairs, to_account_index, "toAccountIndex", &str);
            fetch_stmt!(self, query_pairs, batch_num, "batchNum");

            pagination_fetch_stmts!(self, query_pairs);
        }
        let history_transactions: HistoryTransactions = self.http.get(&url).await?;
        Ok((
            history_transactions.transactions,
            history_transactions.pending_items,
        ))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HistoryTransactions {
    pub pending_items: u64,
    pub transactions: Vec<HistoryTransaction>,
}

#[derive(Deserialize, Debug)]
pub enum L1OrL2 {
    L1,
    L2,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Exit,
    Transfer,
    Deposit,
    CreateAccountDeposit,
    CreateAccountDepositTransfer,
    DepositTransfer,
    ForceTransfer,
    ForceExit,
    TransferToEthAddr,
    TransferToBJJ,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoryTransaction {
    #[serde(rename = "L1orL2")]
    pub l1_or_l2: L1OrL2,
    pub id: String,
    pub item_id: u64,
    pub r#type: TransactionType,
    pub position: u64,
    pub from_account_index: Option<String>,
    pub from_hez_ethereum_address: Option<String>,
    #[serde(rename = "fromBJJ")]
    pub from_bjj: Option<String>,
    pub to_account_index: String,
    pub to_hez_ethereum_address: Option<String>,
    #[serde(rename = "toBJJ")]
    pub to_bjj: Option<String>,
    pub amount: String,
    pub batch_num: Option<u32>,
    #[serde(rename = "historicUSD")]
    pub historic_usd: Option<f64>,
    pub timestamp: String,
    pub token: Token,
    #[serde(rename = "L1Info")]
    pub l1_info: Option<L1Info>,
    #[serde(rename = "L2Info")]
    pub l2_info: Option<L2Info>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct L1Info {
    pub to_forge_l1_transactions_num: Option<u32>,
    pub user_origin: bool,
    pub deposit_amount: String,
    pub deposit_amount_success: bool,
    #[serde(rename = "historicDepositAmountUSD")]
    pub historic_deposit_amount_usd: Option<f64>,
    pub ethereum_block_num: u128,
}

#[derive(Deserialize, Debug)]
pub struct L2Info {
    pub fee: u32,
    #[serde(rename = "historicFeeUSD")]
    pub historic_fee_usd: Option<f64>,
    pub nonce: u128,
}

mod test {

    #[test]
    fn test_transactions_history() {
        let data = r#"{
            "transactions": [
              {
                "L1Info": {
                  "ethereumBlockNum": 3,
                  "historicDepositAmountUSD": null,
                  "depositAmount": "0",
                  "depositAmountSuccess": false,
                  "toForgeL1TransactionsNum": 7,
                  "userOrigin": true
                },
                "L1orL2": "L1",
                "L2Info": null,
                "amount": "88888800000000000",
                "batchNum": 9,
                "fromAccountIndex": "hez:ETH:262",
                "fromBJJ": "hez:Mj_xDCjfN-y3h_4hbhEdtkqnz6LFF1Cf4AV_8IoQswwh",
                "fromHezEthereumAddress": "hez:0x2B5AD5c4795c026514f8317c7a215E218DcCD6cF",
                "historicUSD": 44.4444,
                "id": "0x000000000000000007000300",
                "itemId": 28,
                "position": 3,
                "timestamp": "2020-11-26T09:18:40.004749Z",
                "toAccountIndex": "hez:EXIT:1",
                "toBJJ": null,
                "toHezEthereumAddress": null,
                "token": {
                  "USD": 500,
                  "decimals": 18,
                  "ethereumAddress": "0x0000000000000000000000000000000000000000",
                  "ethereumBlockNum": 0,
                  "fiatUpdate": "2020-11-26T09:18:27.034866Z",
                  "id": 0,
                  "itemId": 1,
                  "name": "Ether",
                  "symbol": "ETH"
                },
                "type": "ForceExit"
              },
              {
                "L1Info": null,
                "L1orL2": "L2",
                "L2Info": {
                  "fee": 123,
                  "historicFeeUSD": 2.15037380962404,
                  "nonce": 1
                },
                "amount": "55555500000000000",
                "batchNum": 8,
                "fromAccountIndex": "hez:TKN1:264",
                "fromBJJ": "hez:Mj_xDCjfN-y3h_4hbhEdtkqnz6LFF1Cf4AV_8IoQswwh",
                "fromHezEthereumAddress": "hez:0x2B5AD5c4795c026514f8317c7a215E218DcCD6cF",
                "historicUSD": 23.4999765,
                "id": "0x020000000001080000000001",
                "itemId": 19,
                "position": 2,
                "timestamp": "2020-11-26T09:18:40.004749Z",
                "toAccountIndex": "hez:TKN1:260",
                "toBJJ": "hez:81h61cx0FKR1RXcLbHW8cZMPY8SR6yKU3ei4pmcLjpaQ",
                "toHezEthereumAddress": "hez:0x6813Eb9362372EEF6200f3b1dbC3f819671cBA69",
                "token": {
                  "USD": 423,
                  "decimals": 18,
                  "ethereumAddress": "0x0000000000000000000000000000000000000064",
                  "ethereumBlockNum": 2,
                  "fiatUpdate": "2020-11-26T09:18:27.04357Z",
                  "id": 1,
                  "itemId": 2,
                  "name": "Test Token 1",
                  "symbol": "TKN1"
                },
                "type": "Transfer"
              },
              {
                "L1Info": null,
                "L1orL2": "L2",
                "L2Info": {
                  "fee": 44,
                  "historicFeeUSD": 0.1973587359744,
                  "nonce": 2
                },
                "amount": "66666600000000000",
                "batchNum": 8,
                "fromAccountIndex": "hez:ETH:259",
                "fromBJJ": "hez:W6x4TZOAZ9mAqdOb3Xm_hKDLspaXfEfMMN4tXOkinS-W",
                "fromHezEthereumAddress": "hez:0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf",
                "historicUSD": 33.3333,
                "id": "0x020000000001030000000002",
                "itemId": 20,
                "position": 3,
                "timestamp": "2020-11-26T09:18:40.004749Z",
                "toAccountIndex": "hez:EXIT:1",
                "toBJJ": null,
                "toHezEthereumAddress": null,
                "token": {
                  "USD": 500,
                  "decimals": 18,
                  "ethereumAddress": "0x0000000000000000000000000000000000000000",
                  "ethereumBlockNum": 0,
                  "fiatUpdate": "2020-11-26T09:18:27.034866Z",
                  "id": 0,
                  "itemId": 1,
                  "name": "Ether",
                  "symbol": "ETH"
                },
                "type": "Exit"
              },
              {
                "L1Info": {
                  "ethereumBlockNum": 3,
                  "historicDepositAmountUSD": 14099.9999859,
                  "depositAmount": "33333333300000000000",
                  "depositAmountSuccess": true,
                  "toForgeL1TransactionsNum": 2,
                  "userOrigin": true
                },
                "L1orL2": "L1",
                "L2Info": null,
                "amount": "0",
                "batchNum": 4,
                "fromAccountIndex": "hez:TKN1:0",
                "fromBJJ": "hez:W6x4TZOAZ9mAqdOb3Xm_hKDLspaXfEfMMN4tXOkinS-W",
                "fromHezEthereumAddress": "hez:0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf",
                "historicUSD": null,
                "id": "0x000000000000000002000000",
                "itemId": 9,
                "position": 0,
                "timestamp": "2020-11-26T09:18:40.004749Z",
                "toAccountIndex": "hez:TKN1:0",
                "toBJJ": null,
                "toHezEthereumAddress": null,
                "token": {
                  "USD": 423,
                  "decimals": 18,
                  "ethereumAddress": "0x0000000000000000000000000000000000000064",
                  "ethereumBlockNum": 2,
                  "fiatUpdate": "2020-11-26T09:18:27.04357Z",
                  "id": 1,
                  "itemId": 2,
                  "name": "Test Token 1",
                  "symbol": "TKN1"
                },
                "type": "CreateAccountDeposit"
              },
              {
                "L1Info": null,
                "L1orL2": "L2",
                "L2Info": {
                  "fee": 2,
                  "historicFeeUSD": 3.87833366166246e-17,
                  "nonce": 1
                },
                "amount": "11111100000000000",
                "batchNum": 7,
                "fromAccountIndex": "hez:TKN1:261",
                "fromBJJ": "hez:W6x4TZOAZ9mAqdOb3Xm_hKDLspaXfEfMMN4tXOkinS-W",
                "fromHezEthereumAddress": "hez:0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf",
                "historicUSD": 4.6999953,
                "id": "0x020000000001050000000001",
                "itemId": 15,
                "position": 2,
                "timestamp": "2020-11-26T09:18:40.004749Z",
                "toAccountIndex": "hez:TKN1:264",
                "toBJJ": "hez:Mj_xDCjfN-y3h_4hbhEdtkqnz6LFF1Cf4AV_8IoQswwh",
                "toHezEthereumAddress": "hez:0x2B5AD5c4795c026514f8317c7a215E218DcCD6cF",
                "token": {
                  "USD": 423,
                  "decimals": 18,
                  "ethereumAddress": "0x0000000000000000000000000000000000000064",
                  "ethereumBlockNum": 2,
                  "fiatUpdate": "2020-11-26T09:18:27.04357Z",
                  "id": 1,
                  "itemId": 2,
                  "name": "Test Token 1",
                  "symbol": "TKN1"
                },
                "type": "Transfer"
              }
            ],
            "pendingItems": 23
          }"#;
        let _structure: super::HistoryTransactions = serde_json::from_str(data).unwrap();
    }
}
