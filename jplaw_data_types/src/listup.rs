//! リストアップしたデータの一覧
//!
use serde::{Deserialize, Serialize};

/// 法令の略称情報
/// <https://elaws.e-gov.go.jp/abb/>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AbbInfo {
  pub num: String,
  pub abbs: Vec<String>,
}

/// 法令のインデックス情報
pub type LawInfo = crate::law::LawInfo;

/// 条例のインデックス情報
pub type JoreiInfo = crate::jorei::JoreiInfo;
/// 条例の詳細データ
pub type JoreiData = crate::jorei::JoreiData;

/// 判例のインデックス情報
pub type PrecedentInfo = crate::precedent::PrecedentInfo;
/// 判例の詳細データ
pub type PrecedentData = crate::precedent::PrecedentData;
