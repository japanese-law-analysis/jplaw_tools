//! 条文に関する型と関数の定義
//!

use japanese_law_xml_schema::{
  article::{Article, ChapterContents, PartContents, SectionContents, SubsectionContents},
  article_number::ArticleNumber,
  law::{LawBody, MainProvisionContents},
  paragraph::Paragraph,
};
use serde::{Deserialize, Serialize};

/// 条文の位置を示す
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleIndex {
  /// 法令・条例の中身などが書かれたファイル名に起因する一意のID
  pub file_id: String,
  /// 法令・条例名
  pub law_name: String,
  /// 条番号
  pub article_number: ArticleNumber,
  pub part_number: Option<ArticleNumber>,
  pub chapter_number: Option<ArticleNumber>,
  pub section_number: Option<ArticleNumber>,
  pub subsection_number: Option<ArticleNumber>,
  pub division_number: Option<ArticleNumber>,
  /// 附則の場合は付加された改正法の法令番号
  pub suppl_provision_name: Option<String>,
}

/// lawbodyから条文のリストを得る
pub fn article_list_from_lawbody(
  file_id: &str,
  law_name: &str,
  lawbody: &LawBody,
) -> Vec<AnalysisResultInfo<Vec<Paragraph>>> {
  let mut v = Vec::new();
  let mut para_v = Vec::new();
  for main in &lawbody.main_provision.children {
    match main {
      MainProvisionContents::Article(t) => {
        let article_index = ArticleIndex {
          file_id: file_id.to_string(),
          law_name: law_name.to_string(),
          article_number: t.num.clone(),
          part_number: None,
          chapter_number: None,
          section_number: None,
          subsection_number: None,
          division_number: None,
          suppl_provision_name: None,
        };
        v.push(AnalysisResultInfo {
          article_index,
          text_index_opt: None,
          result: t.paragraph.clone(),
        })
      }
      MainProvisionContents::Part(t) => {
        let mut v2 = article_list_from_part(file_id, law_name, Some(&t.num), &t.children);
        v.append(&mut v2);
      }
      MainProvisionContents::Chapter(t) => {
        let mut v2 = article_list_from_chapter(file_id, law_name, None, Some(&t.num), &t.children);
        v.append(&mut v2);
      }
      MainProvisionContents::Section(t) => {
        let mut v2 =
          article_list_from_section(file_id, law_name, None, None, Some(&t.num), &t.children);
        v.append(&mut v2);
      }
      MainProvisionContents::Paragraph(t) => {
        para_v.push(t.clone());
      }
    }
  }
  if !para_v.is_empty() {
    let article_index = ArticleIndex {
      file_id: file_id.to_string(),
      law_name: law_name.to_string(),
      article_number: ArticleNumber::zero(),
      part_number: None,
      chapter_number: None,
      section_number: None,
      subsection_number: None,
      division_number: None,
      suppl_provision_name: None,
    };
    v.push(AnalysisResultInfo {
      article_index,
      text_index_opt: None,
      result: para_v,
    })
  }
  v
}

fn article_list_from_part(
  file_id: &str,
  law_name: &str,
  part_number: Option<&ArticleNumber>,
  lst: &[PartContents],
) -> Vec<AnalysisResultInfo<Vec<Paragraph>>> {
  let mut v = Vec::new();
  for contents in lst.iter() {
    match contents {
      PartContents::Chapter(t) => {
        let mut v2 =
          article_list_from_chapter(file_id, law_name, part_number, Some(&t.num), &t.children);
        v.append(&mut v2)
      }
      PartContents::Article(t) => {
        let article_index = ArticleIndex {
          file_id: file_id.to_string(),
          law_name: law_name.to_string(),
          article_number: t.num.clone(),
          part_number: part_number.cloned(),
          chapter_number: None,
          section_number: None,
          subsection_number: None,
          division_number: None,
          suppl_provision_name: None,
        };
        v.push(AnalysisResultInfo {
          article_index,
          text_index_opt: None,
          result: t.paragraph.clone(),
        })
      }
    }
  }
  v
}

fn article_list_from_chapter(
  file_id: &str,
  law_name: &str,
  part_number: Option<&ArticleNumber>,
  chapter_number: Option<&ArticleNumber>,
  lst: &[ChapterContents],
) -> Vec<AnalysisResultInfo<Vec<Paragraph>>> {
  let mut v = Vec::new();
  for contents in lst.iter() {
    match contents {
      ChapterContents::Section(t) => {
        let mut v2 = article_list_from_section(
          file_id,
          law_name,
          part_number,
          chapter_number,
          Some(&t.num),
          &t.children,
        );
        v.append(&mut v2)
      }
      ChapterContents::Article(t) => {
        let article_index = ArticleIndex {
          file_id: file_id.to_string(),
          law_name: law_name.to_string(),
          article_number: t.num.clone(),
          part_number: part_number.cloned(),
          chapter_number: chapter_number.cloned(),
          section_number: None,
          subsection_number: None,
          division_number: None,
          suppl_provision_name: None,
        };
        v.push(AnalysisResultInfo {
          article_index,
          text_index_opt: None,
          result: t.paragraph.clone(),
        })
      }
    }
  }
  v
}

fn article_list_from_section(
  file_id: &str,
  law_name: &str,
  part_number: Option<&ArticleNumber>,
  chapter_number: Option<&ArticleNumber>,
  seciton_number: Option<&ArticleNumber>,
  lst: &[SectionContents],
) -> Vec<AnalysisResultInfo<Vec<Paragraph>>> {
  let mut v = Vec::new();
  for contents in lst.iter() {
    match contents {
      SectionContents::Subsection(t) => {
        let mut v2 = article_list_from_subsection(
          file_id,
          law_name,
          part_number,
          chapter_number,
          seciton_number,
          Some(&t.num),
          &t.children,
        );
        v.append(&mut v2)
      }
      SectionContents::Article(t) => {
        let article_index = ArticleIndex {
          file_id: file_id.to_string(),
          law_name: law_name.to_string(),
          article_number: t.num.clone(),
          part_number: part_number.cloned(),
          chapter_number: chapter_number.cloned(),
          section_number: seciton_number.cloned(),
          subsection_number: None,
          division_number: None,
          suppl_provision_name: None,
        };
        v.push(AnalysisResultInfo {
          article_index,
          text_index_opt: None,
          result: t.paragraph.clone(),
        })
      }
    }
  }
  v
}

fn article_list_from_subsection(
  file_id: &str,
  law_name: &str,
  part_number: Option<&ArticleNumber>,
  chapter_number: Option<&ArticleNumber>,
  seciton_number: Option<&ArticleNumber>,
  subseciton_number: Option<&ArticleNumber>,
  lst: &[SubsectionContents],
) -> Vec<AnalysisResultInfo<Vec<Paragraph>>> {
  let mut v = Vec::new();
  for contents in lst.iter() {
    match contents {
      SubsectionContents::Division(t) => {
        let mut v2 = article_list_from_division(
          file_id,
          law_name,
          part_number,
          chapter_number,
          seciton_number,
          None,
          &t.num,
          &t.children,
        );
        v.append(&mut v2)
      }
      SubsectionContents::Article(t) => {
        let article_index = ArticleIndex {
          file_id: file_id.to_string(),
          law_name: law_name.to_string(),
          article_number: t.num.clone(),
          part_number: part_number.cloned(),
          chapter_number: chapter_number.cloned(),
          section_number: seciton_number.cloned(),
          subsection_number: subseciton_number.cloned(),
          division_number: None,
          suppl_provision_name: None,
        };
        v.push(AnalysisResultInfo {
          article_index,
          text_index_opt: None,
          result: t.paragraph.clone(),
        })
      }
    }
  }
  v
}

#[allow(clippy::too_many_arguments)]
fn article_list_from_division(
  file_id: &str,
  law_name: &str,
  part_number: Option<&ArticleNumber>,
  chapter_number: Option<&ArticleNumber>,
  seciton_number: Option<&ArticleNumber>,
  subseciton_number: Option<&ArticleNumber>,
  division_number: &ArticleNumber,
  lst: &[Article],
) -> Vec<AnalysisResultInfo<Vec<Paragraph>>> {
  let mut v = Vec::new();
  for t in lst.iter() {
    let article_index = ArticleIndex {
      file_id: file_id.to_string(),
      law_name: law_name.to_string(),
      article_number: t.num.clone(),
      part_number: part_number.cloned(),
      chapter_number: chapter_number.cloned(),
      section_number: seciton_number.cloned(),
      subsection_number: subseciton_number.cloned(),
      division_number: Some(division_number.clone()),
      suppl_provision_name: None,
    };
    v.push(AnalysisResultInfo {
      article_index,
      text_index_opt: None,
      result: t.paragraph.clone(),
    })
  }
  v
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
