// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use asimov_module::prelude::{FromStr, String};
use core::error::Error;

#[derive(Clone, Debug, Default)]
pub struct BlockUrl {
    pub network: String,
    pub height: u64,
}

impl FromStr for BlockUrl {
    type Err = url::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        url::Url::parse(input).map(|url| {
            let network = url.host_str().unwrap_or("mainnet").to_owned();
            let path = url.path().strip_prefix('/').expect("should contain a path");
            let height: u64 = path.parse().expect("should be a valid block height");
            Self { network, height }
        })
    }
}

impl BlockUrl {
    pub fn rpc_url(&self) -> &str {
        match self.network.as_str() {
            "mainnet" => "https://ethereum.publicnode.com",
            "sepolia" => "https://ethereum-sepolia.publicnode.com",
            "holesky" => "https://ethereum-holesky.publicnode.com",
            _ => "https://cloudflare-eth.com",
        }
    }

    pub fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let rpc_url = self.rpc_url();
        let hex_height = format!("0x{:x}", self.height);
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [hex_height, true],
            "id": 1
        });
        let mut response = ureq::post(rpc_url).send_json(body)?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }
}
