//! 条文に関する型と関数の定義
//!

use crate::analysis::AnalysisResultInfo;
use japanese_law_xml_schema::{
  article::{Article, ChapterContents, PartContents, SectionContents, SubsectionContents},
  article_number::ArticleNumber,
  class::SentenceOrColumnOrTable,
  contents::ContentsElement,
  law::{LawBody, MainProvisionContents},
  paragraph::{
    Paragraph, Subitem1, Subitem10, Subitem2, Subitem3, Subitem4, Subitem5, Subitem6, Subitem7,
    Subitem8, Subitem9,
  },
  sentence::SentenceElement,
  suppl_provision,
  text::{Text, TextElement},
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
        let mut v2 =
          article_list_from_chapter(file_id, law_name, None, Some(&t.num), None, &t.children);
        v.append(&mut v2);
      }
      MainProvisionContents::Section(t) => {
        let mut v2 = article_list_from_section(
          file_id,
          law_name,
          None,
          None,
          Some(&t.num),
          None,
          &t.children,
        );
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

  for suppl_provision in &lawbody.suppl_provision {
    let suppl_provision_name = &suppl_provision.amend_law_num.clone().unwrap_or_default();
    let mut suppl_para_v = Vec::new();
    for se in suppl_provision.children.iter() {
      match se {
        suppl_provision::SupplProvisionChildrenElement::Article(t) => {
          let article_index = ArticleIndex {
            file_id: file_id.to_string(),
            law_name: law_name.to_string(),
            article_number: t.num.clone(),
            part_number: None,
            chapter_number: None,
            section_number: None,
            subsection_number: None,
            division_number: None,
            suppl_provision_name: Some(suppl_provision_name.clone()),
          };
          v.push(AnalysisResultInfo {
            article_index,
            text_index_opt: None,
            result: t.paragraph.clone(),
          })
        }
        suppl_provision::SupplProvisionChildrenElement::Chapter(t) => {
          let mut v2 = article_list_from_chapter(
            file_id,
            law_name,
            None,
            Some(&t.num),
            Some(suppl_provision_name),
            &t.children,
          );
          v.append(&mut v2);
        }
        suppl_provision::SupplProvisionChildrenElement::Paragraph(t) => {
          suppl_para_v.push(t.clone());
        }
        _ => (),
      }
    }
    if !suppl_para_v.is_empty() {
      let article_index = ArticleIndex {
        file_id: file_id.to_string(),
        law_name: law_name.to_string(),
        article_number: ArticleNumber::zero(),
        part_number: None,
        chapter_number: None,
        section_number: None,
        subsection_number: None,
        division_number: None,
        suppl_provision_name: Some(suppl_provision_name.clone()),
      };
      v.push(AnalysisResultInfo {
        article_index,
        text_index_opt: None,
        result: suppl_para_v,
      })
    }
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
        let mut v2 = article_list_from_chapter(
          file_id,
          law_name,
          part_number,
          Some(&t.num),
          None,
          &t.children,
        );
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
  suppl_provision_name: Option<&String>,
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
          suppl_provision_name,
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
          suppl_provision_name: suppl_provision_name.cloned(),
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
  suppl_provision_name: Option<&String>,
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
          suppl_provision_name,
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
          suppl_provision_name: suppl_provision_name.cloned(),
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
fn article_list_from_subsection(
  file_id: &str,
  law_name: &str,
  part_number: Option<&ArticleNumber>,
  chapter_number: Option<&ArticleNumber>,
  seciton_number: Option<&ArticleNumber>,
  subseciton_number: Option<&ArticleNumber>,
  suppl_provision_name: Option<&String>,
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
          suppl_provision_name,
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
          division_number: subseciton_number.cloned(),
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
  suppl_provision_name: Option<&String>,
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
      suppl_provision_name: suppl_provision_name.cloned(),
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
  pub paragraph: ArticleNumber,
  /// 号の番号を上の階層から並べる
  /// 何もないときは空
  pub items: Vec<ArticleNumber>,
}

/// 段落のリストから文字列のリストとそのインデックスの組を生成する
/// ルビと線は無視し、上付き文字は`^`、下付き文字は`_`で出力する
pub fn text_list_from_paragraph(lst: &[Paragraph]) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for para in lst.iter() {
    let paragraph_num = &para.num;
    let sentence_text = para
      .sentence
      .iter()
      .map(|sentence| sentence_element_to_str(&sentence.contents))
      .collect::<String>();
    v.push((
      TextIndex {
        paragraph: paragraph_num.clone(),
        items: Vec::new(),
      },
      sentence_text,
    ));
    let mut items = Vec::new();
    for item in para.children.iter() {
      let n = &item.num;
      items.push(n.clone());
      // childrenとsentenceの処理をする
      let sentence_str = match &item.sentence {
        SentenceOrColumnOrTable::Sentence(se) => se
          .iter()
          .map(|sentence| sentence_element_to_str(&sentence.contents))
          .collect::<String>(),
        _ => String::new(),
      };
      v.push((
        TextIndex {
          paragraph: paragraph_num.clone(),
          items: items.clone(),
        },
        sentence_str,
      ));
      let mut v2 = text_list_from_subitem1(paragraph_num, items.clone(), &item.children);
      v.append(&mut v2);
      items.pop();
    }
  }
  v
}

fn text_list_from_subitem1(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem1],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem2(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem2(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem2],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem3(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem3(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem3],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem4(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem4(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem4],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem5(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem5(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem5],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem6(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem6(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem6],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem7(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem7(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem7],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem8(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem8(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem8],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem9(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem9(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem9],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem10(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem10(
  para_num: &ArticleNumber,
  items: Vec<ArticleNumber>,
  chldren: &[Subitem10],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      _ => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l,
      },
      sentence_str,
    ));
  }
  v
}

pub fn text_to_str(text: &Text) -> String {
  let mut s = String::new();
  for v in text.contents.iter() {
    match v {
      TextElement::Sub(sub) => s.push_str(&sub.text),
      TextElement::Sup(sup) => s.push_str(&sup.text),
      TextElement::Ruby(ruby) => s.push_str(&text_to_str(&ruby.text)),
      TextElement::Line(_) => (),
      TextElement::Text(str) => s.push_str(str),
    }
  }
  s
}

pub fn sentence_element_to_str(element: &[SentenceElement]) -> String {
  let mut s = String::new();
  for e in element.iter() {
    match e {
      SentenceElement::String(s2) => s.push_str(s2),
      SentenceElement::Sub(s2) => {
        s.push_str("_{");
        s.push_str(&s2.text);
        s.push('}');
      }
      SentenceElement::Sup(s2) => {
        s.push_str("^{");
        s.push_str(&s2.text);
        s.push('}');
      }
      SentenceElement::ArithFormula(arith_formula) => {
        let contents = &arith_formula.contentes.contents;
        for c in contents.iter() {
          match c {
            ContentsElement::String(s2) => s.push_str(s2),
            ContentsElement::Sub(s2) => {
              s.push_str("_{");
              s.push_str(&s2.text);
              s.push('}');
            }
            ContentsElement::Sup(s2) => {
              s.push_str("^{");
              s.push_str(&s2.text);
              s.push('}');
            }
            _ => (),
          }
        }
      }
      _ => (),
    }
  }
  s
}

#[test]
fn check_para_to_text() {
  use japanese_law_xml_schema::{class, paragraph, sentence, text};
  fn text_to_sentence(num: usize, text: &str) -> sentence::Sentence {
    sentence::Sentence {
      contents: vec![sentence::SentenceElement::String(text.to_string())],
      num: Some(num),
      function: None,
      indent: None,
      writing_mode: text::WritingMode::Vertical,
    }
  }
  let para_lst = vec![
    paragraph::Paragraph {
      caption: None,
      paragraph_num: text::Text{contents: Vec::new()},
      amend_provision: Vec::new(),
      class: Vec::new(),
      sentence: vec![sentence::Sentence {
        contents: vec![sentence::SentenceElement::String(
          "被保佐人が次に掲げる行為をするには、その保佐人の同意を得なければならない。ただし、第九条ただし書に規定する行為については、この限りでない。"
            .to_string()
        )],
        num: Some(1),
        function: None,
        indent: None,
        writing_mode: text::WritingMode::Vertical
      }],
      struct_list: Vec::new(),
      children: vec![
        paragraph::Item {
          title: None,
          sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "元本を領収し、又は利用すること。")]),
          children: Vec::new(),
          struct_list: Vec::new(),
          num: ArticleNumber::from_num_str("1").unwrap(),
          delete: false,
          hide: false
        },
        paragraph::Item {
          title: None,
          sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "主たる債務者が法人である場合の次に掲げる者")]),
          children: vec![
            paragraph::Subitem1 {
              title: None,
              sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "主たる債務者の総株主の議決権（株主総会において決議をすることができる事項の全部につき議決権を行使することができない株式についての議決権を除く。以下この号において同じ。）の過半数を有する者")]),
              children: Vec::new(),
              struct_list: Vec::new(),
              num: ArticleNumber::from_num_str("1").unwrap(),
              delete: false,
              hide: false
            }
          ],
          struct_list: Vec::new(),
          num: ArticleNumber::from_num_str("2").unwrap(),
          delete: false,
          hide: false
        },
        paragraph::Item {
          title: None,
          sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "不動産その他重要な財産に関する権利の得喪を目的とする行為をすること。")]),
          children: Vec::new(),
          struct_list: Vec::new(),
          num: ArticleNumber::from_num_str("3").unwrap(),
          delete: false,
          hide: false
        }
      ],
      num: ArticleNumber::from_num_str("1").unwrap(),
      old_style: false,
      old_num: false,
      hide: false,
    },paragraph::Paragraph {
      caption: None,
      paragraph_num: text::Text{contents: Vec::new()},
      amend_provision: Vec::new(),
      class: Vec::new(),
      sentence: vec![sentence::Sentence {
        contents: vec![sentence::SentenceElement::String(
          "家庭裁判所は、第十一条本文に規定する者又は保佐人若しくは保佐監督人の請求により、被保佐人が前項各号に掲げる行為以外の行為をする場合であってもその保佐人の同意を得なければならない旨の審判をすることができる。ただし、第九条ただし書に規定する行為については、この限りでない。"
            .to_string()
        )],
        num: Some(1),
        function: None,
        indent: None,
        writing_mode: text::WritingMode::Vertical
      }],
      struct_list: Vec::new(),
      children: Vec::new(),
      num: ArticleNumber::from_num_str("2").unwrap(),
      old_style: false,
      old_num: false,
      hide: false,
    }
  ];
  let text_lst = text_list_from_paragraph(&para_lst);
  assert_eq!(
    text_lst,
    vec![
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: Vec::new()
        },
        "被保佐人が次に掲げる行為をするには、その保佐人の同意を得なければならない。ただし、第九条ただし書に規定する行為については、この限りでない。".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: vec![ArticleNumber{base_number: 1, eda_numbers: Vec::new(), range_end_numbers: Vec::new()}]
        },
        "元本を領収し、又は利用すること。".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: vec![ArticleNumber{base_number: 2, eda_numbers: Vec::new(), range_end_numbers: Vec::new()}]
        },
        "主たる債務者が法人である場合の次に掲げる者".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: vec![ArticleNumber{base_number: 2, eda_numbers: Vec::new(), range_end_numbers: Vec::new()},
          ArticleNumber{base_number: 1, eda_numbers: Vec::new(), range_end_numbers: Vec::new()}]
        },
        "主たる債務者の総株主の議決権（株主総会において決議をすることができる事項の全部につき議決権を行使することができない株式についての議決権を除く。以下この号において同じ。）の過半数を有する者".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: vec![ArticleNumber{base_number: 3, eda_numbers: Vec::new(), range_end_numbers: Vec::new()}]
        },
        "不動産その他重要な財産に関する権利の得喪を目的とする行為をすること。".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 2,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: Vec::new()
        },
        "家庭裁判所は、第十一条本文に規定する者又は保佐人若しくは保佐監督人の請求により、被保佐人が前項各号に掲げる行為以外の行為をする場合であってもその保佐人の同意を得なければならない旨の審判をすることができる。ただし、第九条ただし書に規定する行為については、この限りでない。".to_string()
      ),
    ]
  );
}
