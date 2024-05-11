use pdf_extract::OutputError;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Pdf2TextError {
  #[error("io error: {0}")]
  IoError(String),
  #[error("pdf read error({0}): {1}")]
  PdfReadError(OutputError, String),
  #[error("pdf panic")]
  PdfPanic,
}

/// 文字列の抽出
pub fn pdf_bytes_to_text(bytes: &[u8]) -> Result<String, Pdf2TextError> {
  let result = std::panic::catch_unwind(|| pdf_extract::extract_text_from_mem(bytes));
  if result.is_err() {
    return Err(Pdf2TextError::PdfPanic);
  }
  let s = pdf_extract::extract_text_from_mem(bytes)
    .map_err(|e| Pdf2TextError::PdfReadError(e, "bytes".to_string()))?;
  Ok(s)
}

/// ファイルからの文字列の抽出
pub fn pdf_file_to_text(path: &str) -> Result<String, Pdf2TextError> {
  let bytes = std::fs::read(path).map_err(|_| Pdf2TextError::IoError(path.to_string()))?;
  let result = std::panic::catch_unwind(|| pdf_extract::extract_text_from_mem(&bytes));
  if result.is_err() {
    return Err(Pdf2TextError::PdfPanic);
  }
  let s = pdf_extract::extract_text_from_mem(&bytes)
    .map_err(|e| Pdf2TextError::PdfReadError(e, path.to_string()))?;
  Ok(s)
}

/// ページ番号や行番号などの削除
pub fn clean_up(text: &str) -> String {
  // 行番号を表す部分の削除
  // 行末に"5 "のようにテキストとして登場するので取り除く
  let line_number_re = Regex::new("^(?<text>.+?)(5|10|15|20|25) $").unwrap();
  // ページ番号
  let page_number_re = Regex::new(r"^-?(\s|　)*[0-9]+(\s|　)*-?$").unwrap();
  let mut s = String::new();
  for line in text.lines() {
    let l = if let Some(caps) = line_number_re.captures(line) {
      caps["text"].to_string()
    } else {
      line.to_string()
    };
    let l = l.trim();
    if !l.is_empty() && !page_number_re.is_match(l) {
      s.push_str(l);
      s.push('\n');
    }
  }
  s
}
