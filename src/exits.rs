use super::http::Http;
use serde::Deserialize;
use url::Url;

use super::ErrorKind;
use super::PaginationOrder;

use super::tokens::Token;

pub struct ExitsGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,
    token_id: Option<u32>,
    hez_ethereum_address: Option<&'a str>,
    bjj: Option<&'a str>,
    account_index: Option<&'a str>,
    batch_num: Option<u32>,
    only_pending_withdraws: Option<bool>,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> ExitsGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,
            token_id: None,
            hez_ethereum_address: None,
            bjj: None,
            account_index: None,
            batch_num: None,
            only_pending_withdraws: None,
            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(token_id, u32);
    setter!(hez_ethereum_address, &'a str);
    setter!(bjj, &'a str);
    setter!(account_index, &'a str);
    setter!(batch_num, u32);
    setter!(only_pending_withdraws, bool);

    pagination_setters!();

    pub async fn fetch(&self) -> Result<(Vec<Exit>, u64), super::ErrorKind> {
        if is_more_than_one_defined!(self.hez_ethereum_address, self.bjj, self.account_index) {
            return Err(ErrorKind::Api(String::from(
                "Get Exits: bjj, hez_ethereum address and account_index are incompatible with each other",
            )));
        }

        if self.token_id.is_some() && self.account_index.is_some() {
            return Err(super::ErrorKind::Api(String::from(
                "Get Exits: token_id and account_index are incompatible with each other",
            )));
        }

        let mut url = self.url.join("exits").unwrap();
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
            fetch_stmt!(self, query_pairs, bjj, "BJJ", &str);
            fetch_stmt!(self, query_pairs, account_index, "accountIndex", &str);
            fetch_stmt!(
                self,
                query_pairs,
                only_pending_withdraws,
                "onlyPendingWithdraws"
            );

            pagination_fetch_stmts!(self, query_pairs);
        }
        let exits: Exits = self.http.get(&url).await?;
        Ok((exits.exits, exits.pending_items))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Exits {
    pub pending_items: u64,
    pub exits: Vec<Exit>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Exit {
    pub batch_num: u32,
    pub account_index: String,
    pub bjj: String,
    pub hez_ethereum_address: String,
    pub item_id: u64,
    pub merkle_proof: MerkleProof,
    pub balance: String, // BigInt
    pub instant_withdraw: Option<u128>,
    pub delayed_withdraw_request: Option<u128>,
    pub delayed_withdraw: Option<u128>,
    pub token: Token,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerkleProof {
    pub root: String,          // BigInt
    pub siblings: Vec<String>, // Vec<BigInt>
    pub old_key: String,       // BigInt
    pub old_value: String,     // BigInt
    pub is_old0: bool,
    pub key: String,   // BigInt
    pub value: String, // BigInt
    pub fnc: u8,
}
