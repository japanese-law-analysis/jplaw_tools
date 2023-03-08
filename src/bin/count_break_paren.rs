use anyhow::Result;
use clap::Parser;
use tokio::fs::*;
use tokio::io::{AsyncWriteExt, BufReader};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// 解析結果を出力するJSONファイルへのpath
  #[clap(short, long)]
  output: String,
  /// 法令XMLファイル群が置かれている作業ディレクトリへのpath
  #[clap(short, long)]
  work: String,
  /// 法令ファイルのインデックス情報が書かれたJSONファイルへのpath
  #[clap(short, long)]
  index_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  Ok(())
}
