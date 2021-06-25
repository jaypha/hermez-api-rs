use super::http::Http;
use serde::Deserialize;
use url::Url;

use super::ErrorKind;
use super::PaginationOrder;

pub struct TokensGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,
    ids: Option<&'a [u32]>,
    symbols: Option<&'a [&'a str]>,
    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> TokensGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,
            ids: None,
            symbols: None,
            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(ids, &'a [u32]);
    setter!(symbols, &'a [&'a str]);

    pagination_setters!();
    /*
        pub fn ids(&mut self, ids: &'a [u32]) -> &mut Self {
            self.ids = Some(ids);
            self
        }

        pub fn symbols(&mut self, symbols: &'a [&'a str]) -> &mut Self {
            self.symbols = Some(symbols);
            self
        }

    */
    pub async fn fetch(&self) -> Result<(Vec<Token>, u64), ErrorKind> {
        let mut url = self.url.join("tokens").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(ids) = &self.ids {
                query_pairs.append_pair("ids", &itertools::join(*ids, ","));
            }

            if let Some(symbols) = &self.symbols {
                query_pairs.append_pair("symbols", &symbols.join(","));
            }

            pagination_fetch_stmts!(self, query_pairs);
        }
        let tokens: Tokens = self.http.get(&url).await?;
        Ok((tokens.tokens, tokens.pending_items))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Tokens {
    pub pending_items: u64,
    pub tokens: Vec<Token>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: u32,
    pub ethereum_address: String,
    pub item_id: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    ethereum_block_num: u128,
    #[serde(rename = "USD")]
    pub usd: Option<f64>,
    pub fiat_update: Option<String>,
}
