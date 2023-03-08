use anyhow::Result;
use tokio::{fs::File, io::AsyncReadExt};

pub mod get_article;


pub async fn get_file_str(info_file_path: &str) -> Result<String> {
  let mut f = File::open(info_file_path).await?;
  let mut buf = Vec::new();
  f.read_to_end(&mut buf).await?;
  let file_str = std::str::from_utf8(&buf)?;
  Ok(file_str.to_string())
}
