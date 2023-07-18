use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;

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
    content: String,
    #[serde(rename = "sampleType")]
    sample_type: SampleType,
    hash: String,
    size: usize,
    coverage: f64,
    log: String,
}

impl Sample {
    pub fn new(
        container_id: &str,
        file_path: &str,
        sample_type: SampleType,
        coverage: f64,
    ) -> Sample {
        // Compute size
        let metadata = fs::metadata(&file_path).unwrap();
        let size = metadata.len() as usize;

        // Compute hash
        let mut file = fs::File::open(&file_path).unwrap();
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            panic!("Failed to read file");
        }
        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        let hash = format!("{:x}", hasher.finalize());

        let content = String::from_utf8(buffer).unwrap();
        Sample {
            container_id: String::from(container_id),
            content,
            sample_type,
            hash,
            size,
            coverage,
            log: String::from("default_log"),
        }
    }
}
