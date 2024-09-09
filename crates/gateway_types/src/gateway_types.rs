use serde::{Deserialize, Serialize};

use crate::errors::GatewayError;

// TODO(Tsabary/Shahak): Populate the data structure used to invoke the gateway.
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayFnOneInput {}

// TODO(Tsabary/Shahak): Populate the data structure used to invoke the gateway.
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayFnTwoInput {}

// TODO(Tsabary/Shahak): Replace with the actual return type of the gateway function.
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayFnOneReturnValue {}

// TODO(Tsabary/Shahak): Replace with the actual return type of the gateway function.
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayFnTwoReturnValue {}

pub type GatewayResult<T> = Result<T, GatewayError>;
