use super::http::Http;
use serde::Deserialize;
use url::Url;

use super::ErrorKind;
use super::PaginationOrder;

use super::tokens::Token;

pub struct AccountsGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,
    hez_ethereum_address: Option<&'a str>,
    bjj: Option<&'a str>,
    token_ids: Option<&'a [u32]>,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> AccountsGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,
            hez_ethereum_address: None,
            bjj: None,
            token_ids: None,
            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(hez_ethereum_address, &'a str);
    setter!(bjj, &'a str);
    setter!(token_ids, &'a [u32]);

    pagination_setters!();
    /*
        pub fn hez_ethereum_address(&mut self, hez_ethereum_address: &'a str) -> &mut Self {
            self.hez_ethereum_address = Some(hez_ethereum_address);
            self
        }

        pub fn bjj(&mut self, bjj: &'a str) -> &mut Self {
            self.bjj = Some(bjj);
            self
        }

        pub fn token_ids(&mut self, token_ids: &'a [u32]) -> &mut Self {
            self.token_ids = Some(token_ids);
            self
        }

        /// Indicates the desired first item (using the item_id property) to be included in the response.
        pub fn from_item(&mut self, from_item: u64) -> &mut Self {
            self.from_item = Some(from_item);
            self
        }

        /// Order of the returned items. Accounts will be ordered by increasing account index.
        pub fn order(&mut self, order: PaginationOrder) -> &mut Self {
            self.order = Some(order);
            self
        }

        /// Maximum number of items to be returned.
        pub fn limit(&mut self, limit: u64) -> &mut Self {
            self.limit = Some(limit);
            self
        }
    */

    pub async fn fetch(&self) -> Result<(Vec<Account>, u64), ErrorKind> {
        if is_more_than_one_defined!(self.hez_ethereum_address, self.bjj) {
            return Err(ErrorKind::Api(String::from(
                "Get Accounts: Cannot specify both an Ethereum address and a BJJ key",
            )));
        }

        let mut url = self.url.join("accounts").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(token_ids) = &self.token_ids {
                query_pairs.append_pair("tokenIds", &itertools::join(*token_ids, ","));
            }

            fetch_stmt!(
                self,
                query_pairs,
                hez_ethereum_address,
                "hezEthereumAddress"
            );
            fetch_stmt!(self, query_pairs, bjj, "BJJ");
            /*
                        if let Some(hez_ethereum_address) = &self.hez_ethereum_address {
                            query_pairs.append_pair("hezEthereumAddress", hez_ethereum_address);
                        }

                        if let Some(bjj) = &self.bjj {
                            query_pairs.append_pair("BJJ", bjj);
                        }
            */
            pagination_fetch_stmts!(self, query_pairs);
        }
        let accounts: Accounts = self.http.get(&url).await?;
        Ok((accounts.accounts, accounts.pending_items))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Accounts {
    pub pending_items: u64,
    pub accounts: Vec<Account>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub item_id: u64,
    pub account_index: String,
    pub nonce: u128,
    pub balance: String, // BigInt
    pub bjj: String,
    pub hez_ethereum_address: String,
    pub token: Token,
}
