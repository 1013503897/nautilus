use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u32)]
pub enum SampleType {
    Normal = 1,
    Crash,
    Timeout,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sample {
    #[serde(rename = "containerId")]
    container_id: String,
    content: Vec<u8>, // Changed from String to Vec<u8>
    #[serde(rename = "sampleType")]
    sample_type: SampleType,
    hash: String,
    size: usize,
    func_coverage: f32,
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
    ) -> Self {
        let size = code.len();

        // Compute hash
        let mut hasher = Sha256::new();
        hasher.update(&code);
        let hash = format!("{:x}", hasher.finalize());
        Sample {
            container_id: String::from(container_id),
            content: code.to_vec(), // Changed from String to Vec<u8>
            sample_type,
            hash,
            size,
            func_coverage,
            lines_coverage,
            log: String::from("default_log"),
        }
    }
}
