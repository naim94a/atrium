// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.sync.getLatestCommit` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///The DID of the repo.
    pub did: crate::types::string::Did,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub cid: crate::types::string::Cid,
    pub rev: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    RepoNotFound(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RepoNotFound(msg) => {
                write!(_f, "RepoNotFound")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
