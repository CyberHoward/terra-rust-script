use crate::error::TerraRustScriptError;
use base64::encode;
use serde_json::{json, Value};
use terra_rust_api::{core_types::Coin, Message, MsgExecuteContract};

pub struct Multisig;

impl Multisig {
    pub fn create_proposal(
        json_msg: &Value,
        _group_name: &str,
        contract_addr: &str,
        multisig_addr: &str,
        sender_addr: &str,
        coins: &[Coin],
    ) -> Result<Message, TerraRustScriptError> {
        let encoded = encode(json_msg.to_string());
        let msg = json!({
          "propose": {
            "msgs": [
              {
                "wasm": {
                  "execute": {
                    "msg": encoded,
                    "funds": coins,
                    "contract_addr": contract_addr
                  }
                }
              }
            ],
            "title": "",
            "description": ""
          }
        });

        log::debug!("{}", msg);
        log::info!(
            "Proposed msgs: {}",
            json!([
              {
                "wasm": {
                  "execute": {
                    "msg": encoded,
                    "funds": coins,
                    "contract_addr": contract_addr
                  }
                }
              }
            ])
        );

        Ok(MsgExecuteContract::create_from_value(
            sender_addr,
            multisig_addr,
            &msg,
            &[],
        )?)
    }
}
