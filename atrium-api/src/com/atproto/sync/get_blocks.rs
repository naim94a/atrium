// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.sync.getBlocks` namespace."]
#[doc = "`com.atproto.sync.getBlocks`"]
#[doc = "Gets blocks from a given repo."]
#[async_trait::async_trait]
pub trait GetBlocks: crate::xrpc::XrpcClient {
    async fn get_blocks(&self, params: Parameters) -> Result<(), Box<dyn std::error::Error>> {
        let _ = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::GET,
            "com.atproto.sync.getBlocks",
            Some(serde_urlencoded::to_string(&params)?),
            None,
            None,
        )
        .await?;
        Ok(())
    }
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub cids: Vec<String>,
    #[doc = "The DID of the repo."]
    pub did: String,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
