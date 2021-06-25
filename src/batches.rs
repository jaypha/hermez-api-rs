use super::http::Http;
use serde::Deserialize;
use url::Url;

use super::ErrorKind;
use super::PaginationOrder;

use super::transactions_history::HistoryTransaction;

use std::collections::HashMap;

pub struct BatchesGetOptions<'a> {
    http: &'a Http,
    url: &'a Url,

    min_batch_num: Option<u32>,
    max_batch_num: Option<u32>,
    slot_num: Option<u32>,
    forger_addr: Option<&'a str>,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> BatchesGetOptions<'a> {
    pub fn new(http: &'a Http, url: &'a Url) -> Self {
        Self {
            http,
            url,

            min_batch_num: None,
            max_batch_num: None,
            slot_num: None,
            forger_addr: None,

            from_item: None,
            order: None,
            limit: None,
        }
    }

    setter!(min_batch_num, u32);
    setter!(max_batch_num, u32);
    setter!(slot_num, u32);
    setter!(forger_addr, &'a str);

    pagination_setters!();

    pub async fn fetch(&self) -> Result<(Vec<Batch>, u64), ErrorKind> {
        let mut url = self.url.join("batches").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            fetch_stmt!(self, query_pairs, min_batch_num, "minBatchNum");
            fetch_stmt!(self, query_pairs, max_batch_num, "maxBatchNum");
            fetch_stmt!(self, query_pairs, slot_num, "slotNum");
            fetch_stmt!(self, query_pairs, forger_addr, "forgerAddr", &str);

            pagination_fetch_stmts!(self, query_pairs);
        }
        let batches: Batches = self.http.get(&url).await?;
        Ok((batches.batches, batches.pending_items))
    }
}

#[derive(Deserialize, Debug)]
pub struct FullBatch {
    pub batch: Batch,
    pub transactions: Vec<HistoryTransaction>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Batches {
    pub pending_items: u64,
    pub batches: Vec<Batch>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    item_id: u64,
    batch_num: u32,
    ethereum_block_num: u128,
    ethereum_block_hash: String,
    ethereum_tx_hash: String,
    timestamp: String,
    forger_addr: String,
    collected_fees: HashMap<String, String>,
    #[serde(rename = "historicTotalCollectedFeesUSD")]
    historic_total_collected_fees_usd: f64,
    state_root: String,
    num_accounts: u64,
    exit_root: String,
    forge_l1_transactions_num: Option<u64>,
    slot_num: u32,
    forged_transactions: u64,
}
