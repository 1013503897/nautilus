use crate::sample::Sample;
use crate::status::Status;
#[allow(unused_imports)]
use log::{debug, error, info, warn};

pub async fn send_samples(addr: &str, samples: Vec<Sample>) {
    let url = format!("http://{}/api/hermitcrab/sample/addSample", addr);
    let client = reqwest::Client::new();

    match client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&samples)
        .send()
        .await
    {
        Ok(res) if res.status().is_success() => {
            info!("Successfully sent samples");
        }
        _ => {
            warn!("Failed to send samples");
        }
    }
}

pub async fn send_sample(addr: &str, sample: &Sample) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/api/hermitcrab/sample/addSample", addr);
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&vec![sample])
        .send()
        .await?;
    if res.status().is_success() {
        info!("Successfully sent samples");
        Ok(())
    } else {
        warn!("Failed to send samples");
        Err("Failed to send samples".into())
    }
}
pub async fn send_status_param(
    addr: &str,
    status_param: &Status,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/api/hermitcrab/taskInstance/updateStatus", addr);
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&status_param)
        .send()
        .await?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err("Failed to send StatusParam".into())
    }
}
