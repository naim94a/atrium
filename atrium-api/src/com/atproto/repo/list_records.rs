// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `com.atproto.repo.listRecords` namespace."]
#[doc = "`com.atproto.repo.listRecords`"]
#[doc = "List a range of records in a collection."]
#[async_trait::async_trait]
pub trait ListRecords: crate::xrpc::XrpcClient {
    async fn list_records(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        let body = crate::xrpc::XrpcClient::send::<Error>(
            self,
            http::Method::GET,
            "com.atproto.repo.listRecords",
            Some(serde_urlencoded::to_string(&params)?),
            None,
            None,
        )
        .await?;
        serde_json::from_slice(&body).map_err(|e| e.into())
    }
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[doc = "The NSID of the record type."]
    pub collection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[doc = "The number of records to return."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The handle or DID of the repo."]
    pub repo: String,
    #[doc = "Reverse the order of the returned records?"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[doc = "DEPRECATED: The highest sort-ordered rkey to stop at (exclusive)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rkey_end: Option<String>,
    #[doc = "DEPRECATED: The lowest sort-ordered rkey to start from (exclusive)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rkey_start: Option<String>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub records: Vec<Record>,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
#[doc = "`com.atproto.repo.listRecords#record`"]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub cid: String,
    pub uri: String,
    pub value: crate::records::Record,
}
