//! 条例に使うデータ構造

use crate::law::Date;
use serde::{Deserialize, Serialize};

/// 条例の一覧のための情報
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct JoreiInfo {
  /// 名前
  pub title: String,
  /// ファイル名にもなるID
  pub id: String,
  pub reiki_id: String,
  /// 県
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prefecture: Option<String>,
  /// 市町村名
  #[serde(skip_serializing_if = "Option::is_none")]
  pub city: Option<String>,
  /// 公布年月日
  #[serde(skip_serializing_if = "Option::is_none")]
  pub announcement_date: Option<Date>,
  /// 改正年月日
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_date: Option<Date>,
}

/// 条例の中身
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct JoreiData {
  #[serde(default)]
  pub collection: Vec<String>,
  #[serde(default)]
  pub collected_date: Vec<String>,
  #[serde(default)]
  pub updated_date: Vec<Date>,
  pub municipality_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prefecture: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub city: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prefecture_kana: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub city_kana: Option<String>,
  pub municipality_type: String,
  pub area: String,
  pub id: String,
  pub reiki_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub h1: Option<String>,
  pub title: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub announcement_date: Option<Date>,
  pub jorei_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_updated_date: Option<Date>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reiki_dates: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reiki_numbers: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub original_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reiki_url: Option<String>,
  pub has_version: bool,
  pub file_type: String,
  #[serde(default)]
  pub h_type: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub collected_date_s: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub announcement_date_s: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_updated_date_s: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_date_s: Option<String>,
}
