use std::sync::Arc;

use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use papyrus_proc_macros::handle_response_variants;
use serde::{Deserialize, Serialize};
use starknet_mempool_infra::component_client::{
    ClientError,
    LocalComponentClient,
    RemoteComponentClient,
};
use starknet_mempool_infra::component_definitions::ComponentRequestAndResponseSender;
use thiserror::Error;

use crate::errors::GatewayError;
use crate::gateway_types::{
    GatewayFnOneInput,
    GatewayFnOneReturnValue,
    GatewayFnTwoInput,
    GatewayFnTwoReturnValue,
    GatewayResult,
};

pub type LocalGatewayClientImpl = LocalComponentClient<GatewayRequest, GatewayResponse>;
pub type RemoteGatewayClientImpl = RemoteComponentClient<GatewayRequest, GatewayResponse>;
pub type GatewayClientResult<T> = Result<T, GatewayClientError>;
pub type GatewayRequestAndResponseSender =
    ComponentRequestAndResponseSender<GatewayRequest, GatewayResponse>;
pub type SharedGatewayClient = Arc<dyn GatewayClient>;

/// Serves as the gateway's shared interface. Requires `Send + Sync` to allow transferring
/// and sharing resources (inputs, futures) across threads.
#[automock]
#[async_trait]
pub trait GatewayClient: Send + Sync {
    async fn gateway_fn_one(
        &self,
        gateway_fn_one_input: GatewayFnOneInput,
    ) -> GatewayClientResult<GatewayFnOneReturnValue>;

    async fn gateway_fn_two(
        &self,
        gateway_fn_two_input: GatewayFnTwoInput,
    ) -> GatewayClientResult<GatewayFnTwoReturnValue>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GatewayRequest {
    GatewayFnOne(GatewayFnOneInput),
    GatewayFnTwo(GatewayFnTwoInput),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GatewayResponse {
    GatewayFnOne(GatewayResult<GatewayFnOneReturnValue>),
    GatewayFnTwo(GatewayResult<GatewayFnTwoReturnValue>),
}

#[derive(Clone, Debug, Error)]
pub enum GatewayClientError {
    #[error(transparent)]
    ClientError(#[from] ClientError),
    #[error(transparent)]
    GatewayError(#[from] GatewayError),
}

#[async_trait]
impl GatewayClient for LocalGatewayClientImpl {
    async fn gateway_fn_one(
        &self,
        gateway_fn_one_input: GatewayFnOneInput,
    ) -> GatewayClientResult<GatewayFnOneReturnValue> {
        let request = GatewayRequest::GatewayFnOne(gateway_fn_one_input);
        let response = self.send(request).await;
        handle_response_variants!(GatewayResponse, GatewayFnOne, GatewayClientError, GatewayError)
    }

    async fn gateway_fn_two(
        &self,
        gateway_fn_two_input: GatewayFnTwoInput,
    ) -> GatewayClientResult<GatewayFnTwoReturnValue> {
        let request = GatewayRequest::GatewayFnTwo(gateway_fn_two_input);
        let response = self.send(request).await;
        handle_response_variants!(GatewayResponse, GatewayFnTwo, GatewayClientError, GatewayError)
    }
}

#[async_trait]
impl GatewayClient for RemoteGatewayClientImpl {
    async fn gateway_fn_one(
        &self,
        gateway_fn_one_input: GatewayFnOneInput,
    ) -> GatewayClientResult<GatewayFnOneReturnValue> {
        let request = GatewayRequest::GatewayFnOne(gateway_fn_one_input);
        let response = self.send(request).await?;
        handle_response_variants!(GatewayResponse, GatewayFnOne, GatewayClientError, GatewayError)
    }

    async fn gateway_fn_two(
        &self,
        gateway_fn_two_input: GatewayFnTwoInput,
    ) -> GatewayClientResult<GatewayFnTwoReturnValue> {
        let request = GatewayRequest::GatewayFnTwo(gateway_fn_two_input);
        let response = self.send(request).await?;
        handle_response_variants!(GatewayResponse, GatewayFnTwo, GatewayClientError, GatewayError)
    }
}
