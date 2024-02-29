// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.queryModerationEvents` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///If specified, only events where all of these labels were added are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_labels: Option<Vec<String>>,
    ///If specified, only events where all of these tags were added are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_tags: Option<Vec<String>>,
    ///If specified, only events with comments containing the keyword are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    ///Retrieve events created after a given timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<crate::types::string::Datetime>,
    ///Retrieve events created before a given timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<crate::types::string::Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<crate::types::string::Did>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    ///If true, only events with comments are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_comment: Option<bool>,
    ///If true, events on all record types (posts, lists, profile etc.) owned by the did are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_user_records: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU8<100u8>>,
    ///If specified, only events where all of these labels were removed are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_labels: Option<Vec<String>>,
    ///If specified, only events where all of these tags were removed are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_types: Option<Vec<String>>,
    ///Sort direction for the events. Defaults to descending order of created at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    ///The types of events (fully qualified string in the format of com.atproto.admin#modEvent<name>) to filter by. If not specified, all events are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub events: Vec<crate::com::atproto::admin::defs::ModEventView>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
