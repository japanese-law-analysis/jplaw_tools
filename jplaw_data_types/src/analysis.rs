use crate::article::{ArticleIndex, TextIndex};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 解析結果を書き出すときの型
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisResultInfo<T: std::fmt::Debug + Clone + std::hash::Hash + PartialEq + Eq> {
  pub article_index: ArticleIndex,
  pub text_index_opt: Option<TextIndex>,
  pub result: T,
}

/// 解析結果を書き出すときの型
#[derive(Debug, Error, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisError<
  E: std::fmt::Debug + Clone + std::hash::Hash + PartialEq + Eq + std::error::Error,
> {
  pub article_index: ArticleIndex,
  pub text_index_opt: Option<TextIndex>,
  pub text: String,
  pub error: E,
}

/// 読み替え規則の解析結果
/// <https://github.com/japanese-law-analysis/analysis_yomikae>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct YomikaeInfo {
  /// 読み替えられる語
  pub before_words: Vec<String>,
  /// 読み替えられた後の単語
  pub after_word: String,
}

/// 読み替え規則の解析の際のエラー
/// <https://github.com/japanese-law-analysis/analysis_yomikae>
#[derive(Debug, Error, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum YomikaeError {
  #[error("Do not analysis table contents")]
  ContentsOfTable,
  #[error("Unmatched parentheses")]
  UnmatchedParen,
  #[error("Unexpected parallel words")]
  UnexpectedParallelWords,
  #[error("Not found yomikae sentence")]
  NotFoundYomikae,
}

/// 略称解析の結果
/// <https://github.com/japanese-law-analysis/analysis_ryakusyou>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ryakusyou {
  /// 略称
  ryakusyou: String,
  /// 正式名称
  seishiki: String,
}

/// 他の法律文書への参照
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Reference {
  /// 参照先の文書名
  pub ref_name: String,
  /// 参照先の条番号
  pub ref_index: Option<ArticleIndex>,
  /// 参照元の文書名
  pub base_name: String,
  /// 参照元の条番号
  pub base_index: Option<ArticleIndex>,
}
