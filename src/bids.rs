use super::http::Http;
use serde::Deserialize;
use url::Url;

use super::ErrorKind;
use super::PaginationOrder;

pub struct BidsGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,

    slot_num: Option<u32>,
    bidder_addr: Option<&'a str>,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> BidsGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,

            slot_num: None,
            bidder_addr: None,

            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(slot_num, u32);
    setter!(bidder_addr, &'a str);

    pagination_setters!();

    pub async fn fetch(&self) -> Result<(Vec<Bid>, u64), ErrorKind> {
        if self.slot_num.is_none() && self.bidder_addr.is_none() {
            return Err(ErrorKind::Api(String::from(
                "get_bids: At least one filter must be specified",
            )));
        }
        let mut url = self.url.join("bids").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            fetch_stmt!(self, query_pairs, slot_num, "slotNum");
            fetch_stmt!(self, query_pairs, bidder_addr, "bidderAddr", &str);

            pagination_fetch_stmts!(self, query_pairs);
        }
        let bids: Bids = self.http.get(&url).await?;
        Ok((bids.bids, bids.pending_items))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Bids {
    pub pending_items: u64,
    pub bids: Vec<Bid>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    pub item_id: u64,
    pub bidder_addr: String,
    pub forger_addr: String,
    pub slot_num: u32,
    #[serde(rename = "URL")]
    pub url: String,
    pub bid_value: String,
    pub ethereum_block_num: u128,
    pub timestamp: String,
}
