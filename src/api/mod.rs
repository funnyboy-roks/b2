use chrono::{serde::ts_milliseconds, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub account_id: String,
    pub api_info: ApiInfo,
    pub application_key_expiration_timestamp: Option<String>,
    pub authorization_token: String,
}

// Array [
//      String("readBucketReplications"),
//      String("shareFiles"),
//      String("readFileLegalHolds"),
//      String("listKeys"),
//      String("writeKeys"),
//      String("writeBucketRetentions"),
//      String("readFiles"),
//      String("deleteKeys"),
//      String("readBucketRetentions"),
//      String("writeFileLegalHolds"),
//      String("writeFileRetentions"),
//      String("deleteFiles"),
//      String("readBucketEncryption"),
//      String("readBuckets"),
//      String("readFileRetentions"),
//      String("writeBuckets"),
//      String("deleteBuckets"),
//      String("writeBucketEncryption"),
//      String("writeBucketReplications"),
//      String("bypassGovernance"),
//      String("listBuckets"),
//      String("listFiles"),
//      String("writeFiles"),
//  ],

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiInfo {
    pub storage_api: StorageApi,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageApi {
    pub absolute_minimum_part_size: u64,
    pub api_url: String,
    pub bucket_id: Option<String>,
    pub bucket_name: Option<String>,
    pub capabilities: Vec<String>,
    pub download_url: String,
    pub info_type: String,
    pub name_prefix: Option<String>,
    pub recommended_part_size: u64,
    pub s3_api_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    pub account_id: String,
    pub bucket_id: String,
    pub bucket_info: serde_json::Value,
    pub bucket_name: String,
    pub bucket_type: String,                // TODO enum
    pub cors_rules: Vec<serde_json::Value>, // TODO
    pub default_server_side_encryption: GenericConfig,
    pub file_lock_configuration: GenericConfig,
    pub lifecycle_rules: Vec<serde_json::Value>, // TODO
    pub options: Vec<String>,
    pub replication_configuration: GenericConfig,
    pub revision: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericConfig {
    pub is_client_authorized_to_read: bool,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub account_id: String,
    pub action: String, // TODO: enum
    pub bucket_id: String,
    pub content_length: u64,
    pub content_md5: String,
    pub content_sha1: String,
    pub content_type: String,
    pub file_id: String,
    pub file_info: serde_json::Value,
    pub file_name: String,
    pub file_retention: GenericConfig,
    pub legal_hold: GenericConfig,
    pub server_side_encryption: ServerSideEncryption,
    #[serde(with = "ts_milliseconds")]
    pub upload_timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerSideEncryption {
    pub algorithm: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub status: u16,
}
