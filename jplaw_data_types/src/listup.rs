//! リストアップしたデータの一覧
//!
use serde::{Deserialize, Serialize};

/// 法令の略称情報
/// <https://elaws.e-gov.go.jp/abb/>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AbbInfo {
  num: String,
  abbs: Vec<String>,
}

/// 法令の情報
pub type LawInfo = crate::law::LawData;

/// 条例の情報
pub type JoreiInfo = crate::jorei::JoreiInfo;

/// 判例の情報
pub type PrecedentInfo = crate::precedent::PrecedentInfo;
