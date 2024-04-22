//! 条文に関する型と関数の定義
//!

use japanese_law_xml_schema::article_number::ArticleNumber;
use serde::{Deserialize, Serialize};

/// 条文の位置を示す
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleIndex {
  /// 法令（条例）番号
  pub law_num: String,
  /// 法令・条例名
  pub law_name: String,
  /// 条番号
  pub article_number: ArticleNumber,
  pub part_number: Option<usize>,
  pub chapter_number: Option<usize>,
  pub section_number: Option<usize>,
  pub subsection_number: Option<usize>,
  pub division_number: Option<usize>,
  /// 附則の場合は付加された改正法の法令番号
  pub suppl_provision_name: Option<String>,
}

/// 条文内でのテキストの位置を示す
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextIndex {
  /// 段落番号
  pub paragraph: usize,
  /// 号の番号を上の階層から並べる
  /// 何もないときは空
  pub items: Vec<usize>,
}

/// 解析結果を書き出すときの型
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisResultInfo<T: std::fmt::Debug + Clone + std::hash::Hash + PartialEq + Eq> {
  pub article_index: ArticleIndex,
  pub text_index_opt: Option<TextIndex>,
  pub result: T,
}
