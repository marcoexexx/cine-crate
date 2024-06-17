use scraper::Html;

use crate::Result;

pub const NEW_RELEASE_QUERY: &str = "#slider2.owl-carousel.owl-theme .item";

pub async fn get_document() -> Result<Html> {
  let client = reqwest::Client::new();
  let response = client
    .get(&format!("https://www.channelmyanmar.to/{}", ""))
    .send()
    .await?
    .text()
    .await?;
  let html = scraper::Html::parse_document(&response);

  Ok(html)
}
