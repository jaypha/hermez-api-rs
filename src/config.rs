use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub hermez: Hermez,
    pub auction: ConfigAuction,
    pub withdrawal_delayer: ConfigWithdrawalDelayer,
    pub chain_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigAuction {
    pub blocks_per_slot: u64,
    pub initial_minimal_bidding: u64,
    pub genesis_block_num: u128,
    #[serde(rename = "tokenHEZ")]
    pub token_hex: String,
    pub hermez_rollup: String,
    pub governance_address: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hermez {
    pub public_constants: Constants,
    pub max_fee_idx_coordinator: u64,
    pub reserved_idx: u64,
    pub exit_idx: u64,
    pub limit_deposit_amount: f64,
    pub limit_l2_transfer_amount: f64,
    pub limit_tokens: u64,
    pub l1_coordinator_total_bytes: u64,
    pub l1_user_total_bytes: u64,
    pub max_l1_user_tx: u64,
    pub max_l1_tx: u64,
    #[serde(rename = "inputSHAConstantBytes")]
    pub input_sha_constant_bytes: u64,
    pub num_buckets: u64,
    pub max_withdrawal_delay: u64,
    pub exchange_multiplier: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Constants {
    #[serde(rename = "tokenHEZ")]
    pub token_hez: String,
    pub absolute_max_l1_l2_batch_timeout: u64,
    pub verifiers: Vec<Verifier>,
    pub hermez_auction_contract: String,
    pub hermez_governance_address: String,
    pub withdraw_delayer_contract: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Verifier {
    pub max_tx: u64,
    pub nlevels: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigWithdrawalDelayer {
    pub max_withdrawal_delay: u64,
    pub max_emergency_mode_time: u64,
    pub hermez_rollup: String,
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_config() {
        let data = r#"{"auction":{"blocksPerSlot":40,"genesisBlockNum":8256000,"governanceAddress":"0x6873d7012eaa33e393aa7bba23712f673d6e5226","hermezRollup":"0x679b11e0229959c1d3d27c9d20529e4c5df7997c","initialMinimalBidding":10000000000000000000,"tokenHEZ":"0x2521bc90b4f5fb9a8d61278197e5ff5cdbc4fbf2"},"chainId":4,"hermez":{"exchangeMultiplier":100000000000000,"exitIdx":1,"inputSHAConstantBytes":18546,"l1CoordinatorTotalBytes":101,"l1UserTotalBytes":78,"limitDepositAmount":321.0,"limitL2TransferAmount":837.0,"limitTokens":4294967296,"maxFeeIdxCoordinator":64,"maxL1Tx":256,"maxL1UserTx":128,"maxWithdrawalDelay":1209600,"numBuckets":0,"publicConstants":{"absoluteMaxL1L2BatchTimeout":240,"hermezAuctionContract":"0x0a8a6d65ad9046c2a57a5ca8bab2ae9c3345316d","hermezGovernanceAddress":"0x6873d7012eaa33e393aa7bba23712f673d6e5226","tokenHEZ":"0x2521bc90b4f5fb9a8d61278197e5ff5cdbc4fbf2","verifiers":[{"maxTx":400,"nlevels":32},{"maxTx":2048,"nlevels":32}],"withdrawDelayerContract":"0xefd96cfbaf1b0dd24d3882b0d6b8d95f85634724"},"reservedIdx":255},"withdrawalDelayer":{"hermezRollup":"0x679b11e0229959c1d3d27c9d20529e4c5df7997c","maxEmergencyModeTime":15724800,"maxWithdrawalDelay":1209600}}"#;

        let _structure: Config = serde_json::from_str(data).unwrap();
    }
}
