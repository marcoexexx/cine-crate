pub use error::{Error, Result};

use model::Video;
use scraper::Selector;
use utils::{get_document, NEW_RELEASE_QUERY};

mod error;
pub mod model;
pub mod utils;

macro_rules! resolve_element {
  ($el:expr, $q:expr) => {{
    let selector = Selector::parse("a")?;

    $el.select(&selector).nth(0).unwrap().text().next().unwrap_or("")
  }};

  ($el:expr, $q:expr, attr <- ($($attr:expr),*)) => {{
    let selector = Selector::parse("a")?;

    let el = $el.select(&selector).nth(0).unwrap().value();

    ($(el.attr(stringify!($attr)).unwrap_or(""),)*)
  }};
}

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
