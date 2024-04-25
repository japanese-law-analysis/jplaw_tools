//! ファイルIOとログの出力を行う

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tracing::subscriber::SetGlobalDefaultError;
use tracing::{error, info, warn};

#[derive(Debug, Error)]
pub enum IoError {
  #[error("io error: {0}")]
  Io(io::Error),
  #[error("not utf8")]
  Utf8,
  #[error("parse value")]
  Parse,
  #[error("write to json")]
  WriteJson,
}

/// ファイルパスからデータを読み取る関数
pub async fn read_value<T: DeserializeOwned + Clone>(file: &str) -> Result<T, IoError> {
  let mut f = File::open(file).await.map_err(IoError::Io)?;
  let mut buf = Vec::new();
  f.read_to_end(&mut buf).await.map_err(IoError::Io)?;
  let s = std::string::String::from_utf8(buf).map_err(|_| IoError::Utf8)?;
  let v = &serde_json::from_str::<T>(&s).map_err(|_| IoError::Parse)?;
  Ok(v.clone())
}

/// リスト形式のデータを書き出す先のファイルを生成する関数
pub async fn gen_file_value_lst(path: &str) -> Result<File, IoError> {
  let mut f = File::create(path).await.map_err(IoError::Io)?;
  f.write_all(b"[\n").await.map_err(IoError::Io)?;
  Ok(f)
}

/// ファイルに対してデータを一つずつ書き出す関数
pub async fn write_value_lst<T: Serialize>(file: &mut File, t: T) -> Result<(), IoError> {
  let s = serde_json::to_string(&t).map_err(|_| IoError::WriteJson)?;
  let len = file.metadata().await.map_err(IoError::Io)?.len();
  if len >= 3 {
    file.write_all(b",\n").await.map_err(IoError::Io)?;
  }
  file.write_all(s.as_bytes()).await.map_err(IoError::Io)?;
  Ok(())
}

/// 書き出し終わったファイルを閉じる関数
pub async fn flush_file_value_lst(file: &mut File) -> Result<(), IoError> {
  file.write_all(b"\n]\n").await.map_err(IoError::Io)?;
  file.flush().await.map_err(IoError::Io)?;
  Ok(())
}

#[tokio::test]
async fn test() {
  let mut f = gen_file_value_lst("tests/test.json").await.unwrap();
  for v in 100..200 {
    write_value_lst(&mut f, v).await.unwrap();
  }
  flush_file_value_lst(&mut f).await.unwrap();
}

/// ログの出力制御
pub async fn init_logger() -> Result<(), SetGlobalDefaultError> {
  let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber)?;
  Ok(())
}

/// 処理の開始を知らせる
pub fn start_log<T: Debug>(message: &str, value: &T) {
  info!("[START] {message}: {value:?}")
}

/// 処理の終了を知らせる
pub fn end_log<T: Debug>(message: &str, value: &T) {
  info!("[END] {message}: {value:?}")
}

/// 何かしらの情報を出すログ
pub fn info_log<T: Debug>(message: &str, value: &T) {
  info!("[INFO] {message}: {value:?}")
}

/// 警告のログ
pub fn wran_log<T: Debug>(message: &str, value: &T) {
  warn!("[WARNING] {message}: {value:?}")
}

/// エラーのログ
pub fn error_log<T: Debug>(message: &str, value: &T) {
  error!("[ERROR] {message}: {value:?}")
}
