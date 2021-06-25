use super::http::Http;
use serde::Deserialize;
use url::Url;

use super::ErrorKind;
use super::PaginationOrder;

use super::bids::Bid;

pub struct SlotsGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,

    min_slot_num: Option<u32>,
    max_slot_num: Option<u32>,
    won_by_ethereum_address: Option<&'a str>,
    finished_auction: Option<bool>,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> SlotsGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,

            min_slot_num: None,
            max_slot_num: None,
            won_by_ethereum_address: None,
            finished_auction: None,

            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(min_slot_num, u32);
    setter!(max_slot_num, u32);
    setter!(won_by_ethereum_address, &'a str);
    setter!(finished_auction, bool);

    pagination_setters!();

    pub async fn fetch(&self) -> Result<(Vec<Slot>, u64), ErrorKind> {
        let mut url = self.url.join("slots").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            fetch_stmt!(self, query_pairs, min_slot_num, "minSlotNum");
            fetch_stmt!(self, query_pairs, max_slot_num, "maxSlotNum");
            fetch_stmt!(
                self,
                query_pairs,
                won_by_ethereum_address,
                "wonByEthereumAddress",
                &str
            );
            fetch_stmt!(self, query_pairs, finished_auction, "finishedAuction");

            pagination_fetch_stmts!(self, query_pairs);
        }
        let slots: Slots = self.http.get(&url).await?;
        Ok((slots.slots, slots.pending_items))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Slots {
    pub pending_items: u64,
    pub slots: Vec<Slot>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub item_id: u64,
    pub slot_num: u32,
    pub first_block: u128,
    pub last_block: u128,
    pub open_auction: bool,
    pub best_bid: Option<Bid>,
}
