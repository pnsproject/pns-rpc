use std::sync::Arc;

use codec::Codec;
use futures::FutureExt;
use jsonrpc_core::{Error as RpcError, ErrorCode};
use jsonrpc_derive::rpc;

use sp_block_builder::BlockBuilder;
use sp_blockchain::HeaderBackend;

use sp_runtime::{generic::BlockId, traits};

pub use self::gen_client::Client as SystemClient;

pub use rpc_def::PnsRpcApi;

/// Future that resolves to account nonce.
type FutureResult<T> = jsonrpc_core::BoxFuture<Result<T, RpcError>>;

/// System RPC methods.
#[rpc]
pub trait PnsApi<BlockHash, AccountId, Node, Balance, Duration> {
    #[rpc(name = "pns_queryNodes")]
    fn query_nodes(&self, owner: AccountId, at: Option<BlockHash>) -> FutureResult<Vec<Node>>;

    #[rpc(name = "pns_renewPrice")]
    fn renew_price(
        &self,
        name_len: u8,
        duration: Duration,
        at: Option<BlockHash>,
    ) -> FutureResult<Balance>;

    #[rpc(name = "pns_registryPrice")]
    fn registry_price(
        &self,
        name_len: u8,
        duration: Duration,
        at: Option<BlockHash>,
    ) -> FutureResult<Balance>;

    #[rpc(name = "pns_registerFee")]
    fn register_fee(&self, name_len: u8, at: Option<BlockHash>) -> FutureResult<Balance>;

    #[rpc(name = "pns_queryOperators")]
    fn query_operators(
        &self,
        caller: AccountId,
        at: Option<BlockHash>,
    ) -> FutureResult<Vec<AccountId>>;

    #[rpc(name = "pns_checkExpiresRegistrable")]
    fn check_expires_registrable(&self, node: Node, at: Option<BlockHash>) -> FutureResult<bool>;

    #[rpc(name = "pns_checkExpiresRenewable")]
    fn check_expires_renewable(&self, node: Node, at: Option<BlockHash>) -> FutureResult<bool>;

    #[rpc(name = "pns_checkExpiresUseable")]
    fn check_expires_useable(&self, node: Node, at: Option<BlockHash>) -> FutureResult<bool>;
}

/// Error type of this RPC api.
pub enum Error {
    /// The transaction was not decodable.
    DecodeError,
    /// The call to runtime failed.
    RuntimeError,
}

impl From<Error> for i64 {
    fn from(e: Error) -> i64 {
        match e {
            Error::RuntimeError => 1,
            Error::DecodeError => 2,
        }
    }
}

/// An implementation of System-specific RPC methods on full client.
pub struct FullPns<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> FullPns<C, B> {
    /// Create new `FullSystem` given client and transaction pool.
    pub fn new(client: Arc<C>) -> Self {
        FullPns {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, Node, Balance, Duration>
    PnsApi<<Block as traits::Block>::Hash, AccountId, Node, Balance, Duration> for FullPns<C, Block>
where
    C: sp_api::ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C: Send + Sync + 'static,
    C::Api: PnsRpcApi<Block, AccountId, Node, Balance, Duration>,
    C::Api: BlockBuilder<Block>,
    Block: traits::Block,
    AccountId: Clone + std::fmt::Display + Codec + Send + 'static,
    Node: Clone + std::fmt::Display + Codec + Send + 'static,
    Balance: Clone + std::fmt::Display + Codec + Send + 'static,
    Duration: Clone + std::fmt::Display + Codec + Send + 'static,
{
    fn query_nodes(
        &self,
        owner: AccountId,
        at: Option<<Block as traits::Block>::Hash>,
    ) -> FutureResult<Vec<Node>> {
        let get_nodes = || {
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

            let nodes = api.query_nodes(&at, owner).map_err(|e| RpcError {
                code: ErrorCode::ServerError(Error::RuntimeError.into()),
                message: "Unable to query nodes.".into(),
                data: Some(format!("{:?}", e).into()),
            })?;

            Ok(nodes)
        };

        let res = get_nodes();
        async move { res }.boxed()
    }

    fn renew_price(
        &self,
        name_len: u8,
        duration: Duration,
        at: Option<<Block as traits::Block>::Hash>,
    ) -> FutureResult<Balance> {
        let get_price = || {
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

            let price = api
                .renew_price(&at, name_len, duration)
                .map_err(|e| RpcError {
                    code: ErrorCode::ServerError(Error::RuntimeError.into()),
                    message: "Unable to query renew price.".into(),
                    data: Some(format!("{:?}", e).into()),
                })?;

            Ok(price)
        };

        let res = get_price();
        async move { res }.boxed()
    }

    fn registry_price(
        &self,
        name_len: u8,
        duration: Duration,
        at: Option<<Block as traits::Block>::Hash>,
    ) -> FutureResult<Balance> {
        let get_price = || {
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

            let price = api
                .registry_price(&at, name_len, duration)
                .map_err(|e| RpcError {
                    code: ErrorCode::ServerError(Error::RuntimeError.into()),
                    message: "Unable to query registry price.".into(),
                    data: Some(format!("{:?}", e).into()),
                })?;

            Ok(price)
        };

        let res = get_price();
        async move { res }.boxed()
    }

    fn register_fee(
        &self,
        name_len: u8,
        at: Option<<Block as traits::Block>::Hash>,
    ) -> FutureResult<Balance> {
        let get_price = || {
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

            let price = api.register_fee(&at, name_len).map_err(|e| RpcError {
                code: ErrorCode::ServerError(Error::RuntimeError.into()),
                message: "Unable to query register fee.".into(),
                data: Some(format!("{:?}", e).into()),
            })?;

            Ok(price)
        };

        let res = get_price();
        async move { res }.boxed()
    }

    fn query_operators(
        &self,
        caller: AccountId,
        at: Option<<Block as traits::Block>::Hash>,
    ) -> FutureResult<Vec<AccountId>> {
        let get_operators = || {
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

            let operators = api.query_operators(&at, caller).map_err(|e| RpcError {
                code: ErrorCode::ServerError(Error::RuntimeError.into()),
                message: "Unable to query nodes.".into(),
                data: Some(format!("{:?}", e).into()),
            })?;

            Ok(operators)
        };

        let res = get_operators();
        async move { res }.boxed()
    }

    fn check_expires_registrable(
        &self,
        node: Node,
        at: Option<<Block as traits::Block>::Hash>,
    ) -> FutureResult<bool> {
        let get_res = || {
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

            let res = api
                .check_expires_registrable(&at, node)
                .map_err(|e| RpcError {
                    code: ErrorCode::ServerError(Error::RuntimeError.into()),
                    message: "Unable to check expires registrable.".into(),
                    data: Some(format!("{:?}", e).into()),
                })?;

            Ok(res)
        };

        let res = get_res();
        async move { res }.boxed()
    }

    fn check_expires_renewable(
        &self,
        node: Node,
        at: Option<<Block as traits::Block>::Hash>,
    ) -> FutureResult<bool> {
        let get_res = || {
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

            let res = api
                .check_expires_renewable(&at, node)
                .map_err(|e| RpcError {
                    code: ErrorCode::ServerError(Error::RuntimeError.into()),
                    message: "Unable to check expires renewable.".into(),
                    data: Some(format!("{:?}", e).into()),
                })?;

            Ok(res)
        };

        let res = get_res();
        async move { res }.boxed()
    }

    fn check_expires_useable(
        &self,
        node: Node,
        at: Option<<Block as traits::Block>::Hash>,
    ) -> FutureResult<bool> {
        let get_res = || {
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or(self.client.info().best_hash));

            let res = api.check_expires_useable(&at, node).map_err(|e| RpcError {
                code: ErrorCode::ServerError(Error::RuntimeError.into()),
                message: "Unable to check expires useable.".into(),
                data: Some(format!("{:?}", e).into()),
            })?;

            Ok(res)
        };

        let res = get_res();
        async move { res }.boxed()
    }
}
