//! 裁判例ページのデータ
//!

use crate::law::Date;
use serde::{Deserialize, Serialize};

/// 裁判の種類
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrialType {
  /// 最高裁判所
  SupremeCourt,
  /// 高等裁判所
  HighCourt,
  /// 下級裁判所
  LowerCourt,
  /// 行政事件
  AdministrativeCase,
  /// 労働事件
  LaborCase,
  /// 知的財産
  IPCase,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecedentInfo {
  /// 事件番号
  pub case_number: String,
  /// 裁判所・部・法廷名
  pub court_name: String,
  /// 裁判の種類
  pub trial_type: TrialType,
  /// 裁判年月日
  pub date: Date,
  /// 事件に振られているID
  pub lawsuit_id: String,
}

impl PrecedentInfo {
  /// データの入ったファイル名を生成する
  pub fn file_name(&self) -> String {
    format!(
      "{}_{}_{:?}_{}.json",
      self.case_number, self.court_name, self.trial_type, self.lawsuit_id
    )
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecedentData {
  /// 裁判の種類
  pub trial_type: TrialType,
  /// 裁判年月日
  pub date: Date,
  /// 事件番号
  pub case_number: String,
  /// 事件名
  pub case_name: String,
  /// 裁判所・部・法廷名
  pub court_name: String,
  /// 争われた対象の権利の種別
  #[serde(skip_serializing_if = "Option::is_none")]
  pub right_type: Option<String>,
  /// 訴訟類型
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lawsuit_type: Option<String>,
  /// 裁判種別
  #[serde(skip_serializing_if = "Option::is_none")]
  pub result_type: Option<String>,
  /// 結果
  #[serde(skip_serializing_if = "Option::is_none")]
  pub result: Option<String>,
  /// 判例集等巻・号・頁
  #[serde(skip_serializing_if = "Option::is_none")]
  pub article_info: Option<String>,
  /// 原審裁判所名
  #[serde(skip_serializing_if = "Option::is_none")]
  pub original_court_name: Option<String>,
  /// 原審事件番号
  #[serde(skip_serializing_if = "Option::is_none")]
  pub original_case_number: Option<String>,
  /// 原審裁判年月日
  #[serde(skip_serializing_if = "Option::is_none")]
  pub original_date: Option<Date>,
  /// 原審結果
  #[serde(skip_serializing_if = "Option::is_none")]
  pub original_result: Option<String>,
  /// 分野
  #[serde(skip_serializing_if = "Option::is_none")]
  pub field: Option<String>,
  /// 判示事項の要旨
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gist: Option<String>,
  /// 裁判要旨
  #[serde(skip_serializing_if = "Option::is_none")]
  pub case_gist: Option<String>,
  /// 参照法条
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ref_law: Option<String>,
  /// 事件に振られているID
  pub lawsuit_id: String,
  /// 詳細が乗っているページ
  pub detail_page_link: String,
  /// 判決文全文のPDFリンク
  pub full_pdf_link: String,
  /// 判決文全文
  #[serde(skip_serializing_if = "Option::is_none")]
  pub contents: Option<String>,
}
