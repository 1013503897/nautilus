use crate::queue::QueueItem;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha2::{Digest, Sha256};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u32)]
pub enum SampleType {
    Normal = 1,
    Crash,
    Timeout,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sample {
    #[serde(rename = "ast")]
    queue_item: Option<QueueItem>,
    #[serde(rename = "containerId")]
    container_id: String,
    content: String, // Changed from String to Vec<u8>
    #[serde(rename = "sampleType")]
    sample_type: SampleType,
    hash: String,
    size: usize,
    #[serde(rename = "functionCoverage")]
    func_coverage: f32,
    #[serde(rename = "lineCoverage")]
    lines_coverage: f32,
    log: String,
}

impl Sample {
    pub fn new(
        container_id: &str,
        code: &[u8],
        sample_type: SampleType,
        func_coverage: f32,
        lines_coverage: f32,
        queue_item: Option<QueueItem>,
    ) -> Self {
        let size = code.len();

        // Compute hash
        let mut hasher = Sha256::new();
        hasher.update(&code);
        let hash = format!("{:x}", hasher.finalize());
        Sample {
            container_id: String::from(container_id),
            content: String::from_utf8(code.to_vec()).unwrap(),
            sample_type,
            hash,
            size,
            func_coverage,
            lines_coverage,
            log: String::from("default_log"),
            queue_item,
        }
    }
}
