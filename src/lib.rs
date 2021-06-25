//! Idiomatic interface to the Hermez v1 API
//!
//! The Hermez node API is the layer that allows 3rd party apps and services to interface with the node to use the layer two features of the Hermez rollup. Example of these apps are:
//!
//! * Wallet: send L2 transactions, check balance, ...
//! * Explorer: List transactions, slots, batches, ...
//! * Exchange integrations
//!
//! Note that some of the interactions with the rollup must be done using the Ethereum network directly. Another way to integrate with the rollup is to deploy a node and connect directly to its PostgreSQL database.
//!
//! # Usage
//!
//! All access is done though a `HermezApi` object. Create an instance by calling
//! HermezApi::new() with a valid base URL for the node.
//!
//! The API provides three URL contants that can be used.
//! * MAINNET_URL: Use with Mainnet
//! * TESTNET_URL: Use with the Rinkeby test net
//! * LOCALHOST_URL: Use with a locally hosted node. Use for unit tests.
//!
//! # Examples
//!
//! ```no_run
//! use hermez_api::HermezApi;
//!
//! let api_test = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
//! let api_other = HermezApi::new("https://some.other.domain/hermez").unwrap();
//! ```
//!
//! # Useful Links
//! [API Reference](https://apidoc.hermez.network/ "Hermez API Reference")
//!
//! [Hermez Home](https://hermez.io)
//!

use url::{ParseError, Url};

use std::fmt;

mod http;
mod macros;

use http::Http;

pub mod account_creation_authorization;
pub mod accounts;
pub mod batches;
pub mod bids;
pub mod config;
pub mod coordinators;
pub mod exits;
pub mod health;
pub mod slots;
pub mod state;
pub mod tokens;
pub mod transactions_history;
pub mod transactions_pool;

use account_creation_authorization::{AccountCreationAuthorization, PostAccoutCreation, Success};
use accounts::{Account, AccountsGetOptions};
use batches::{Batch, BatchesGetOptions, FullBatch};
use bids::BidsGetOptions;
use config::Config;
use coordinators::CoordinatorsGetOptions;
use exits::ExitsGetOptions;
use health::Health;
use slots::{Slot, SlotsGetOptions};
use state::State;
use tokens::Token;
use tokens::TokensGetOptions;
use transactions_history::{HistoryTransaction, TransactionsHistoryGetOptions};
use transactions_pool::{
    PoolL2Transaction, TransactionsPoolGetOptions, TransactionsPoolPostOptions,
};

//----------------------------------------------------------------------------

/// Pagination order used with most requests
pub enum PaginationOrder {
    Asc,
    Desc,
}

impl fmt::Display for PaginationOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Asc => "ASC",
                Self::Desc => "DESC",
            }
        )
    }
}

//----------------------------------------------------------------------------

/// Encapsulates the verious errors that can be encountered
#[derive(Debug)]
pub enum ErrorKind {
    /// Request returned an error, supplies a HTTP response code, and a description.
    Http(u16, String),
    /// The request itself failed.
    Transport,
    /// JSON error
    Json(String),
    /// Error produced by the API.
    Api(String),
}

//----------------------------------------------------------------------------

/// Maintains a connection to Hermez API server, and serves as an interface to it.
///
/// # Example
///
/// ```no_run
/// use hermez_api::HermezApi;
///
/// let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
/// let state = api.get_state().await.unwrap();
///
/// println!("{}", state.network.current_slot);
/// ```
pub struct HermezApi {
    url: Url,
    http: Http,
}

impl HermezApi {
    /// Base URL for the Hermez node on Mainnet
    pub const MAINNET_URL: &'static str = "https://api.hermez.io";

    /// Base URL for the Hermez node on Rinkeby
    pub const TESTNET_URL: &'static str = "https://api.testnet.hermez.io";

    /// Base URL for the Hermez node on localhost, usefull for testing changes locally and required for nuit tests
    pub const LOCALHOST_URL: &'static str = "http://localhost:8086";

    /// API version
    pub const VERSION: &'static str = "v1";

    /// Constructs a new HermezApi instance
    /// # Examples
    ///
    /// ```no_run
    /// use hermez_api::HermezApi;
    ///
    /// let api_test = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    /// let api_other = HermezApi::new("https://some.other.domain/hermez").unwrap();
    /// ```
    pub fn new(base_api_url: &str) -> Result<Self, ParseError> {
        let url_prefix = format!("{}/{}/", base_api_url, Self::VERSION);
        let url = Url::parse(&url_prefix)?;
        Ok(Self {
            url,
            http: http::Http::new(),
        })
    }

    //-----------------------------------------------------

    /// Returns the health status of the hermez node
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hermez_api::HermezApi;
    ///
    /// let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    /// let health = api.get_health().await.unwrap();
    ///
    /// println!("{}, {}", health.status, health.version);
    /// ```
    pub async fn get_health(&self) -> Result<Health, ErrorKind> {
        self.http.get(&self.url.join("health").unwrap()).await
    }

    /// Returns a builder for the bids endpoint.
    ///
    /// It's necessary to provide at least one of the filers: slot_num or
    /// bidder_addr.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hermez_api::HermezApi;
    ///
    /// let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    /// let (bids, pending_items) = api.bids_get_options().slot_num(784).fetch().await.unwrap();
    ///
    /// println!("{:?}", bids);
    /// ```
    pub fn bids_get_options(&self) -> BidsGetOptions<'_> {
        BidsGetOptions::new(&self.http, &self.url)
    }

    /// Return constant configuration of the network
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hermez_api::HermezApi;
    ///
    /// let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    /// let config = api.get_config().await.unwrap();
    ///
    /// println!("{:?}", config);
    /// ```
    pub async fn get_config(&self) -> Result<Config, ErrorKind> {
        self.http.get(&self.url.join("config").unwrap()).await
    }

    /// Get information of a token supported by Hermez Network.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hermez_api::HermezApi;
    ///
    /// let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    /// let token = api.get_token().await.unwrap();
    ///
    /// println!("{:?}", config);
    /// ```
    pub async fn get_token(&self, token_id: u32) -> Result<Token, ErrorKind> {
        self.http
            .get(&self.url.join(&format!("tokens/{}", token_id)).unwrap())
            .await
    }

    /// Get information of the supported tokens in the Hermez Network.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hermez_api::HermezApi;
    ///
    /// let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    /// let (tokens, pending_items) = api.tokens_get_options().ids(&[2,44,689].fetch().await.unwrap();
    ///
    /// println!("{:?}", tokens);
    /// ```
    pub fn tokens_get_options(&self) -> TokensGetOptions<'_> {
        TokensGetOptions::new(&self.http, &self.url)
    }

    /// Get an account by its index.
    pub async fn get_account(&self, account_index: &str) -> Result<Account, ErrorKind> {
        self.http
            .get(
                &self
                    .url
                    .join(&format!("accounts/{}", account_index))
                    .unwrap(),
            )
            .await
    }

    /// Get account balances and other associated information.
    ///
    /// The following parameters are incompatible with one another (cannot set more than one).
    /// * hez_ethereum_address and bjj
    pub fn accounts_get_options(&self) -> AccountsGetOptions<'_> {
        AccountsGetOptions::new(&self.http, &self.url)
    }

    /// Get specific exit information.
    ///
    /// Get exit information form a specific exit tree and account. This information is required to perform a withdraw. Exits are identified with accounIndex and batchNum since every batch that has exits has a different exit tree.
    pub async fn get_exit(
        &self,
        batch_num: u32,
        account_index: &str,
    ) -> Result<Account, ErrorKind> {
        self.http
            .get(
                &self
                    .url
                    .join(&format!("exits/{}/{}", batch_num, account_index))
                    .unwrap(),
            )
            .await
    }

    /// Get exit information. This information is required to perform a withdraw.
    ///
    /// The following parameters are incompatible with one another (cannot set more than one)
    /// * bjj, hez_ethereum address and account_index
    /// * token_id and account_index
    pub fn exits_get_options(&self) -> ExitsGetOptions<'_> {
        ExitsGetOptions::new(&self.http, &self.url)
    }

    /// Get details and status of a historical transaction.
    ///
    /// Get transaction by its ID. This endpoint will return all the different
    /// types of transactions except those that are still in the pool of any
    /// coordinator.
    pub async fn get_transaction_history(&self, id: &str) -> Result<HistoryTransaction, ErrorKind> {
        self.http
            .get(
                &self
                    .url
                    .join(&format!("transaction-history/{}", id))
                    .unwrap(),
            )
            .await
    }

    /// Get details and status of transactions that have been forged.
    ///
    /// Get historical transactions. This endpoint will return all the different
    /// types of forged transactions, this means that:
    /// * Transactions that are still in the transaction pool of any coordinator are not included. These transactions can be fetched using HermezApi::get_transactions_pool().
    /// * L1 transactions sent by users that have not been forged yet are not included. These transactions can be fetched using HermezApi::get_transactions_history().
    pub fn transactions_history_get_options(&self) -> TransactionsHistoryGetOptions<'_> {
        TransactionsHistoryGetOptions::new(&self.http, &self.url)
    }

    /// Get a specific batch.
    pub async fn get_batch(&self, batch_num: u32) -> Result<Batch, ErrorKind> {
        self.http
            .get(&self.url.join(&format!("batches/{}", batch_num)).unwrap())
            .await
    }

    /// Get a full batch
    ///
    /// Get a specific batch, including the associated transactions.
    /// The object returned in this method can be a bit heavy. If you're
    /// devloping a front end, you may consider using a combinaton of get_batch()
    /// and get_transaction_history.
    pub async fn get_full_batch(&self, batch_num: u32) -> Result<FullBatch, ErrorKind> {
        self.http
            .get(
                &self
                    .url
                    .join(&format!("full-batches/{}", batch_num))
                    .unwrap(),
            )
            .await
    }

    /// Get information about forged batches.
    pub fn batches_get_options(&self) -> BatchesGetOptions<'_> {
        BatchesGetOptions::new(&self.http, &self.url)
    }

    /// Get information about a specific slot.
    pub async fn get_slot(&self, slot_num: u32) -> Result<Slot, ErrorKind> {
        self.http
            .get(&self.url.join(&format!("slots/{}", slot_num)).unwrap())
            .await
    }

    /// Get information about slots.
    pub fn slots_get_options(&self) -> SlotsGetOptions<'_> {
        SlotsGetOptions::new(&self.http, &self.url)
    }

    /// Return information that represents the current state of the network.
    ///
    /// Also includes metrics and statistics.
    ///
    /// # Example
    ///
    /// ```
    /// use hermez_api::HermezApi;
    ///
    /// let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
    /// let state = api.get_state().await.unwrap();
    ///
    /// println!("{}", state.network.current_slot);
    /// ```
    pub async fn get_state(&self) -> Result<State, ErrorKind> {
        self.http.get(&self.url.join("state").unwrap()).await
    }

    /// Get information about coordinators.
    pub fn coordinators_get_options(&self) -> CoordinatorsGetOptions<'_> {
        CoordinatorsGetOptions::new(&self.http, &self.url)
    }

    pub async fn get_account_creation_authorization(
        &self,
        hez_ethereum_address: &str,
    ) -> Result<AccountCreationAuthorization, ErrorKind> {
        self.http
            .get(
                &self
                    .url
                    .join(&format!(
                        "account-creation-authorization/{}",
                        hez_ethereum_address
                    ))
                    .unwrap(),
            )
            .await
    }

    pub async fn post_account_creation_authorization(
        &self,
        hez_ethereum_address: &str,
        bjj: &str,
        signature: &str,
    ) -> Result<(), ErrorKind> {
        self.http
            .post::<PostAccoutCreation, Success>(
                &self
                    .url
                    .join(&format!("account-creation-authorization"))
                    .unwrap(),
                &PostAccoutCreation {
                    hez_ethereum_address,
                    bjj,
                    signature,
                },
            )
            .await?;
        Ok(())
    }

    pub async fn get_transactions_pool(&self, id: &str) -> Result<PoolL2Transaction, ErrorKind> {
        self.http
            .get(&self.url.join(&format!("transactions_pool/{}", id)).unwrap())
            .await
    }

    pub fn transactions_pool_get_options(&self) -> TransactionsPoolGetOptions<'_> {
        TransactionsPoolGetOptions::new(&self.http, &self.url)
    }

    pub fn transactions_pool_post_options(&self) -> TransactionsPoolPostOptions<'_> {
        TransactionsPoolPostOptions::new(&self.http, &self.url)
    }
}

pub mod prelude {
    pub use super::HermezApi;
    #[doc(no_inline)]
    pub use url::ParseError;
}

#[cfg(test)]
mod tests {
    use super::HermezApi;

    #[test]
    fn test_make_api() {
        const API_BASE_URL: &str = "https://api.testnet.hermez.io";
        const API_VERSION: &str = "v1";

        let api = HermezApi::new(HermezApi::TESTNET_URL).unwrap();

        assert_eq!(
            format!("{}/{}/", API_BASE_URL, API_VERSION),
            api.url.as_str()
        );
    }

    #[test]
    fn test_make_api_with_bad_url() {
        assert!(HermezApi::new("xyz").is_err());
    }
}
