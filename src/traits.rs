use async_trait::async_trait;
use serde_json::Value;
use terra_rust_api::{client::tx_types::TXResultSync, core_types::Coin};

use crate::error::TerraRustScriptError;

pub trait ContractInterface {
    type I: serde::Serialize;
    type E: serde::Serialize;
    type Q: serde::Serialize;
    type M: serde::Serialize;
}

#[async_trait(?Send)]
pub trait ContractAPI<T: ContractInterface> {
    async fn execute(
        &self,
        exec_msg: &<T as ContractInterface>::E,
        coins: Vec<Coin>,
    ) -> Result<TXResultSync, TerraRustScriptError>;

    async fn instantiate(
        &self,
        init_msg: &<T as ContractInterface>::I,
        admin: Option<String>,
        coins: Vec<Coin>,
    ) -> Result<TXResultSync, TerraRustScriptError>;

    async fn query(
        &self,
        query_msg: &<T as ContractInterface>::Q,
    ) -> Result<Value, TerraRustScriptError>;

    async fn upload(
        &self,
        name: &str,
        path: Option<&str>,
    ) -> Result<TXResultSync, TerraRustScriptError>;
}
