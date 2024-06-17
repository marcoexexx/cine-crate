pub use error::{Error, Result};

use model::Video;
use scraper::Selector;
use utils::{get_document, NEW_RELEASE_QUERY};

mod error;
pub mod model;
mod resolve_element;
pub mod utils;

pub async fn new_release() -> Result<Vec<Video>> {
  let mut new_releases = Vec::new();

  let document = get_document().await?;
  let new_release_selector = Selector::parse(NEW_RELEASE_QUERY)?;
  let items = document.select(&new_release_selector);

  for video in items {
    let (cm_link,) = resolve_element!(&video, "a", attr <- ("href"));
    let title = resolve_element!(&video, "span.ttps");
    let (poster,) = resolve_element!(&video, "a img", attr <- ("src"));

    new_releases.push(Video {
      cm_link: cm_link.to_string(),
      slug: cm_link.split("/").nth(3).unwrap_or("").to_owned(),
      title: title.to_string(),
      poster: poster.to_string(),
      description: String::default(),
      casts: Vec::default(),
      photos: Vec::default(),
    })
  }

  Ok(new_releases)
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[tokio::test]
  async fn test_get_random() {
    new_release().await.unwrap();
  }
}
