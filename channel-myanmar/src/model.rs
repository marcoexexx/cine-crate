use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Video {
  pub title: String,
  pub poster: String,
  pub slug: String,
  pub cm_link: String,
  pub description: String,
  pub photos: Vec<String>,
  pub casts: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DownloadLink {
  pub protocol: String,
  pub link: String,
  pub size: String,
  pub quality: String,
}
