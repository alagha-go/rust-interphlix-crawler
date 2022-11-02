pub async fn get_request(url: String) -> Result<String, reqwest::Error> {
    let res = crate::CLIENT.get(&url).send().await?.text().await?;
    Ok(res)
}