use crate::sample::Sample;

pub async fn send_samples(
    addr: &str,
    samples: &Vec<Sample>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/api/sample/addSample", addr);
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&samples)
        .send()
        .await?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err("Failed to send samples".into())
    }
}

pub async fn send_sample(addr: &str, sample: &Sample) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/api/sample/addSample", addr);
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&sample)
        .send()
        .await?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err("Failed to send samples".into())
    }
}
