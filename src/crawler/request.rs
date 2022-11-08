pub async fn get_request(url: String) -> crate::Result<String> {
    let res = crate::CLIENT.get(&url).send().await?.text().await?;
    Ok(res)
}