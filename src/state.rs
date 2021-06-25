use serde::Deserialize;

use super::batches::Batch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub network: StateNetwork,
    pub metrics: StateMetrics,
    pub rollup: StateRollup,
    pub auction: StateAuction,
    pub withdrawal_delayer: StateWithdrawalDelayer,
    pub recommended_fee: RecommededFee,
    pub node: Node,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateNetwork {
    pub last_ethereum_block: u128,
    pub last_synched_block: u128,
    pub last_batch: Batch,
    pub current_slot: u32,
    pub next_forgers: Vec<NextForger>,
    pub pending_l1_transactions: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateAuction {
    pub ethereum_block_num: u128,
    pub boot_coordinator: String,
    pub boot_coordinator_url: String,
    pub slot_deadline: u64,
    pub closed_auction_slots: u64,
    pub open_auction_slots: u64,
    pub default_slot_set_bid: Vec<String>,
    pub default_slot_set_bid_slot_num: u64,
    pub outbidding: f64,
    pub donation_address: String,
    pub allocation_ratio: Vec<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateMetrics {
    pub transactions_per_batch: f64,
    pub batch_frequency: f64,
    pub transactions_per_second: f64,
    pub token_accounts: u64,
    pub wallets: u64,
    pub avg_transaction_fee: f64,
    pub estimated_time_to_forge_l1: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NextForger {
    pub coordinator: Coordinator,
    pub period: Period,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub slot_num: u32,
    pub from_block: u128,
    pub to_block: u128,
    pub from_timestamp: String,
    pub to_timestamp: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub forge_delay: f64,
    pub pool_load: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecommededFee {
    pub existing_account: f64,
    pub create_account: f64,
    pub create_account_internal: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateRollup {
    pub ethereum_block_num: u128,
    pub forge_l1_l2_batch_timeout: u64,
    pub fee_add_token: String,
    pub withdrawal_delay: u64,
    pub buckets: Vec<Bucket>,
    pub safe_mode: bool, // ?
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    #[serde(rename = "ceilUSD")]
    pub ceil_usd: String,
    pub block_stamp: String,
    pub withdrawals: String,
    pub rate_blocks: String,
    pub rate_withdrawals: String,
    pub max_withdrawals: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateWithdrawalDelayer {
    pub ethereum_block_num: u128,
    pub hermez_governance_address: String,
    pub emergency_council_address: String,
    pub withdrawal_delay: u128,
    pub emergency_mode_starting_block: u64,
    pub emergency_mode: bool,
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn test_state() {
        let data = r#"{"auction":{"allocationRatio":[4000,4000,2000],"bootCoordinator":"0x4fc28cd8d35b6fd644e5c1822d67609c11e137f2","bootCoordinatorUrl":"https://api.testnet.hermez.io","closedAuctionSlots":2,"defaultSlotSetBid":["10000000000000000000","10000000000000000000","10000000000000000000","10000000000000000000","10000000000000000000","10000000000000000000"],"defaultSlotSetBidSlotNum":0,"donationAddress":"0xa9bccc0e1349a2a4d0dc995b6d76a39bf094752f","ethereumBlockNum":0,"openAuctionSlots":4320,"outbidding":1000,"slotDeadline":20},"metrics":{"avgTransactionFee":0.7683524487369349,"batchFrequency":724.6034482758621,"estimatedTimeToForgeL1":933.8359621451104,"tokenAccounts":3296,"transactionsPerBatch":11.327586206896552,"transactionsPerSecond":0.015632807480905134,"wallets":2532},"network":{"currentSlot":12329,"lastBatch":{"batchNum":3820,"collectedFees":{"0":"410593952760602"},"ethereumBlockHash":"0x9dd9dc91874f36bdb70f4c96cb7cfdd81ff416c708b7e4d88d1377be8cab0b6b","ethereumBlockNum":8749128,"ethereumTxHash":"0x9b00b1e21614bf81f94bccee363b7c707734aa945c416664b1cdc4c339c77f0c","exitRoot":"9243650862400386652935443853420782774488625238281045820552471640906651173234","forgeL1TransactionsNum":3478,"forgedTransactions":3,"forgerAddr":"0x4fc28cd8d35b6fd644e5c1822d67609c11e137f2","historicTotalCollectedFeesUSD":0.9372628159666262,"itemId":4156,"numAccounts":1,"slotNum":12328,"stateRoot":"8959502156778784348673995263085607054258651793097544964467155317341246690646","timestamp":"2021-06-12T04:45:21Z"},"lastEthereumBlock":8749164,"lastSynchedBlock":8749164,"nextForgers":[{"coordinator":{"URL":"https://api.testnet.hermez.io","bidderAddr":"0x0000000000000000000000000000000000000000","ethereumBlock":0,"forgerAddr":"0x4fc28cd8d35b6fd644e5c1822d67609c11e137f2","itemId":0},"period":{"fromBlock":8749160,"fromTimestamp":"2021-06-12T04:53:21Z","slotNum":12329,"toBlock":8749199,"toTimestamp":"2021-06-12T05:03:06Z"}},{"coordinator":{"URL":"https://api.testnet.hermez.io","bidderAddr":"0x0000000000000000000000000000000000000000","ethereumBlock":0,"forgerAddr":"0x4fc28cd8d35b6fd644e5c1822d67609c11e137f2","itemId":0},"period":{"fromBlock":8749200,"fromTimestamp":"2021-06-12T05:03:21Z","slotNum":12330,"toBlock":8749239,"toTimestamp":"2021-06-12T05:13:06Z"}},{"coordinator":{"URL":"https://api.testnet.hermez.io","bidderAddr":"0x0000000000000000000000000000000000000000","ethereumBlock":0,"forgerAddr":"0x4fc28cd8d35b6fd644e5c1822d67609c11e137f2","itemId":0},"period":{"fromBlock":8749240,"fromTimestamp":"2021-06-12T05:13:21Z","slotNum":12331,"toBlock":8749279,"toTimestamp":"2021-06-12T05:23:06Z"}}],"pendingL1Transactions":7},"node":{"forgeDelay":600,"poolLoad":0},"recommendedFee":{"createAccount":0.5,"createAccountInternal":0.5,"existingAccount":0.5},"rollup":{"buckets":[{"blockStamp":"0","ceilUSD":"100","maxWithdrawals":"1000","rateBlocks":"40","rateWithdrawals":"1000","withdrawals":"998"},{"blockStamp":"0","ceilUSD":"1000","maxWithdrawals":"100","rateBlocks":"40","rateWithdrawals":"100","withdrawals":"99"},{"blockStamp":"0","ceilUSD":"10000","maxWithdrawals":"10","rateBlocks":"40","rateWithdrawals":"10","withdrawals":"9"},{"blockStamp":"0","ceilUSD":"200000","maxWithdrawals":"1","rateBlocks":"40","rateWithdrawals":"1","withdrawals":"0"},{"blockStamp":"0","ceilUSD":"79228162514264337593543950335","maxWithdrawals":"0","rateBlocks":"1","rateWithdrawals":"0","withdrawals":"0"}],"ethereumBlockNum":8721524,"feeAddToken":"200000000000000000000000000","forgeL1L2BatchTimeout":16,"safeMode":false,"withdrawalDelay":259200},"withdrawalDelayer":{"emergencyCouncilAddress":"0x0a2e0eb98df29318d1e6de32f0ea29fa1b4da134","emergencyMode":false,"emergencyModeStartingBlock":0,"ethereumBlockNum":0,"hermezGovernanceAddress":"0x6873d7012eaa33e393aa7bba23712f673d6e5226","withdrawalDelay":3600}}"#;

        let _structure: State = serde_json::from_str(data).unwrap();
    }
}
