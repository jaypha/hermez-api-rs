use super::http::Http;
use serde::Deserialize;
use url::Url;

use super::ErrorKind;
use super::PaginationOrder;

pub struct CoordinatorsGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,

    forger_addr: Option<&'a str>,
    bidder_addr: Option<&'a str>,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> CoordinatorsGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,

            forger_addr: None,
            bidder_addr: None,

            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(forger_addr, &'a str);
    setter!(bidder_addr, &'a str);

    pagination_setters!();

    pub async fn fetch(&self) -> Result<(Vec<Coordinator>, u64), ErrorKind> {
        let mut url = self.url.join("coordinators").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            fetch_stmt!(self, query_pairs, forger_addr, "forgerAddr", &str);
            fetch_stmt!(self, query_pairs, bidder_addr, "bidderAddr", &str);

            pagination_fetch_stmts!(self, query_pairs);
        }
        let coordinators: Coordinators = self.http.get(&url).await?;
        Ok((coordinators.coordinators, coordinators.pending_items))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Coordinators {
    pub pending_items: u64,
    pub coordinators: Vec<Coordinator>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coordinator {
    pub item_id: u64,
    pub forger_addr: String,
    pub bidder_addr: String,
    #[serde(rename = "URL")]
    pub url: String,
    pub ethereum_block: u128,
}
