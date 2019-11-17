use super::proto;
use exonum::crypto::Hash;

/// Stores content's hash and some metadata about it.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, ProtobufConvert)]
#[exonum(pb = "proto::Timestamp")]
pub struct Timestamp {
    /// Hash of the content.
    pub content_hash: Hash,

    /// Additional metadata.
    pub metadata: String,
}

impl Timestamp {
    /// Create new Timestamp.
    pub fn new(&content_hash: &Hash, metadata: &str) -> Self {
        Self {
            content_hash,
            metadata: metadata.to_owned(),
        }
    }
}
