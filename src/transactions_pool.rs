use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

use super::http::Http;
use super::ErrorKind;
use super::PaginationOrder;

use super::tokens::Token;
use super::transactions_history::TransactionType;

#[derive(Serialize, Deserialize, Debug)]
pub enum PoolL2TransactionState {
    #[serde(rename = "pend")]
    Pend,
    #[serde(rename = "fing")]
    Fing,
    #[serde(rename = "fged")]
    Fged,
    #[serde(rename = "invl")]
    Invl,
}

impl fmt::Display for PoolL2TransactionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pend => "pend",
                Self::Fing => "fing",
                Self::Fged => "fged",
                Self::Invl => "invl",
            }
        )
    }
}

pub struct TransactionsPoolGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,

    state: Option<PoolL2TransactionState>,
    token_id: Option<u32>,
    account_index: Option<&'a str>,
    from_account_index: Option<&'a str>,
    to_account_index: Option<&'a str>,
    r#type: Option<TransactionType>,
    hez_ethereum_address: Option<&'a str>,
    from_hez_ethereum_address: Option<&'a str>,
    to_hez_ethereum_address: Option<&'a str>,
    bjj: Option<&'a str>,
    from_bjj: Option<&'a str>,
    to_bjj: Option<&'a str>,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> TransactionsPoolGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,

            state: None,
            token_id: None,
            account_index: None,
            from_account_index: None,
            to_account_index: None,
            r#type: None,
            hez_ethereum_address: None,
            from_hez_ethereum_address: None,
            to_hez_ethereum_address: None,
            bjj: None,
            from_bjj: None,
            to_bjj: None,

            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(state, PoolL2TransactionState);
    setter!(token_id, u32);
    setter!(account_index, &'a str);
    setter!(from_account_index, &'a str);
    setter!(to_account_index, &'a str);
    setter!(r#type, TransactionType);
    setter!(hez_ethereum_address, &'a str);
    setter!(from_hez_ethereum_address, &'a str);
    setter!(to_hez_ethereum_address, &'a str);
    setter!(bjj, &'a str);
    setter!(from_bjj, &'a str);
    setter!(to_bjj, &'a str);

    pagination_setters!();

    pub async fn fetch(&self) -> Result<(Vec<PoolL2Transaction>, u64), ErrorKind> {
        if is_more_than_one_defined!(
            self.account_index,
            self.from_account_index,
            self.to_account_index,
            self.hez_ethereum_address,
            self.from_hez_ethereum_address,
            self.to_hez_ethereum_address,
            self.bjj,
            self.from_bjj,
            self.to_bjj
        ) {
            return Err(ErrorKind::Api(String::from(
                "Transactions Pool: hez_ethereum address, from_hez_ethereum address, to_hez_ethereum address, bjj, from_bjj, to_bjj, account_index, from_account_index and to_account_index are incompatible with each other",
            )));
        }

        let mut url = self.url.join("transactions_pool").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            fetch_stmt!(self, query_pairs, state, "state");
            fetch_stmt!(self, query_pairs, token_id, "tokenId");
            fetch_stmt!(self, query_pairs, account_index, "accountIndex", &str);
            fetch_stmt!(
                self,
                query_pairs,
                from_account_index,
                "fromAccountIndex",
                &str
            );
            fetch_stmt!(self, query_pairs, to_account_index, "toAccountIndex", &str);
            fetch_stmt!(self, query_pairs, r#type, "type", enum);
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
            fetch_stmt!(self, query_pairs, bjj, "bjj", &str);
            fetch_stmt!(self, query_pairs, from_bjj, "fromBjj", &str);
            fetch_stmt!(self, query_pairs, to_bjj, "toBjj", &str);

            pagination_fetch_stmts!(self, query_pairs);
        }
        let transactions_pool: PoolL2Transactions = self.http.get(&url).await?;
        Ok((
            transactions_pool.transactions_pool,
            transactions_pool.pending_items,
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionTypeL2 {
    Exit,
    Transfer,
    TransferToEthAddr,
    TransferToBJJ,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PoolL2Transactions {
    pub pending_items: u64,
    pub transactions_pool: Vec<PoolL2Transaction>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PoolL2Transaction {
    pub id: String,
    pub r#type: TransactionTypeL2,
    pub from_account_index: String,
    pub from_hez_ethereum_address: Option<String>,
    #[serde(rename = "fromBJJ")]
    pub from_bjj: Option<String>,
    pub to_account_index: String,
    pub to_hez_ethereum_address: Option<String>,
    #[serde(rename = "toBJJ")]
    pub to_bjj: Option<String>,
    pub amount: String,
    pub fee: u16,
    pub nonce: u128,
    pub state: PoolL2TransactionState,
    pub info: String,
    pub request_from_account_index: String,
    pub request_from_hez_ethereum_address: Option<String>,
    #[serde(rename = "requestFromBJJ")]
    pub request_from_bjj: Option<String>,
    pub request_to_account_index: Option<String>,
    pub request_to_hez_ethereum_address: Option<String>,
    #[serde(rename = "requestToBJJ")]
    pub request_to_bjj: Option<String>,
    pub request_amount: Option<String>,
    pub request_fee: Option<u16>,
    pub request_nonce: Option<u128>,
    pub token: Token,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostPoolL2Transaction<'a> {
    pub id: Option<&'a str>,
    pub r#type: Option<TransactionTypeL2>,
    pub token_id: Option<u32>,
    pub from_account_index: Option<&'a str>,
    pub to_account_index: Option<&'a str>,
    pub to_hez_ethereum_address: Option<&'a str>,
    pub to_bjj: Option<&'a str>,
    pub amount: Option<&'a str>,
    pub fee: Option<u16>,
    pub nonce: Option<u128>,
    pub signature: Option<&'a str>,

    pub request_from_account_index: Option<&'a str>,
    pub request_to_account_index: Option<&'a str>,
    pub request_to_hez_ethereum_address: Option<&'a str>,
    pub request_to_bjj: Option<&'a str>,
    pub request_token_id: Option<u32>,
    pub request_amount: Option<&'a str>,
    pub request_fee: Option<u16>,
    pub request_nonce: Option<u128>,
}

pub struct TransactionsPoolPostOptions<'a> {
    http: &'a Http,
    url: &'a Url,

    body: PostPoolL2Transaction<'a>,
}

impl<'a> TransactionsPoolPostOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,
            body: Default::default(),
        }
    }

    setter_body!(id, &'a str);
    setter_body!(r#type, TransactionTypeL2);
    setter_body!(token_id, u32);
    setter_body!(from_account_index, &'a str);
    setter_body!(amount, &'a str);
    setter_body!(fee, u16);
    setter_body!(nonce, u128);
    setter_body!(signature, &'a str);

    setter_body!(to_account_index, &'a str);
    setter_body!(to_hez_ethereum_address, &'a str);
    setter_body!(to_bjj, &'a str);

    setter_body!(request_from_account_index, &'a str);
    setter_body!(request_to_account_index, &'a str);
    setter_body!(request_to_hez_ethereum_address, &'a str);
    setter_body!(request_to_bjj, &'a str);
    setter_body!(request_token_id, u32);
    setter_body!(request_amount, &'a str);
    setter_body!(request_fee, u16);
    setter_body!(request_nonce, u128);

    pub async fn fetch(&self) -> Result<String, ErrorKind> {
        test_required!(self, id);

        let url = self.url.join("transactions_pool").unwrap();
        let transaction_id: String = self.http.post(&url, &self.body).await?;
        Ok(transaction_id)
    }
}
