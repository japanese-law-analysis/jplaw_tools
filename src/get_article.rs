use anyhow::Result;
use encoding_rs::UTF_8;
use quick_xml::{
  encoding,
  events::{attributes::Attributes, Event},
  Reader,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum GetArticleError {
  #[error("encoding error at {0:?} : {1:?}")]
  Encoding(Chapter, Vec<u8>),
  #[error("not found attribute at {0:?} : {1}")]
  NotFoundAttribute(Chapter, String),
  #[error("attribuite error")]
  AttributeError,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct LawParagraph {
  /// 法令番号
  pub num: String,
  /// 見出しと章番号
  pub chapter_data: Vec<Chapter>,
}

/// 章・節などを表す
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize, Serialize)]
pub struct Chapter {
  /// 編
  #[serde(skip_serializing_if = "Option::is_none")]
  pub part: Option<usize>,
  /// 章
  #[serde(skip_serializing_if = "Option::is_none")]
  pub chapter: Option<usize>,
  /// 節
  #[serde(skip_serializing_if = "Option::is_none")]
  pub section: Option<usize>,
  /// 款
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subsection: Option<usize>,
  /// 目
  #[serde(skip_serializing_if = "Option::is_none")]
  pub division: Option<usize>,
  /// 条
  pub article: String,
  /// 項
  #[serde(skip_serializing_if = "Option::is_none")]
  pub paragraph: Option<String>,
  /// 号
  #[serde(skip_serializing_if = "Option::is_none")]
  pub item: Option<String>,
  /// イロハなど（深さも記録する）
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sub_item: Option<(usize, String)>,
  /// 附則の場合につける
  #[serde(skip_serializing_if = "Option::is_none")]
  pub suppl_provision_title: Option<String>,
}

fn find_attribute_str(
  attributes: &mut Attributes,
  key: &str,
  chap: &Chapter,
) -> Result<String, GetArticleError> {
  let utf8 = UTF_8;
  let v = attributes.find(|res| match res {
    Err(_) => false,
    Ok(att) => encoding::decode(att.key.0, utf8)
      .map(|s| s == key)
      .unwrap_or(false),
  });
  match v {
    None => Err(GetArticleError::NotFoundAttribute(
      chap.clone(),
      key.to_string(),
    )),
    Some(Err(_)) => Err(GetArticleError::NotFoundAttribute(
      chap.clone(),
      key.to_string(),
    )),
    Some(Ok(v)) => {
      let value = v.value.as_ref();
      encoding::decode(value, utf8)
        .map(|v| v.to_string())
        .map_err(|_| GetArticleError::Encoding(chap.clone(), value.to_vec()))
    }
  }
}

/// 条件に従う条文の条番号等のデータを保存する。
pub async fn search_xml(
  f: fn(&str) -> bool,
  xml_buf: &[u8],
) -> Result<LawParagraph, GetArticleError> {
  let utf8 = UTF_8;

  let mut lst = vec![];
  let mut buf = Vec::new();
  let mut chapter_num = Chapter::default();
  let mut law_num = String::new();
  let mut is_law_num_mode = false;

  let mut reader = Reader::from_reader(xml_buf);
  reader.trim_text(true);
  loop {
    match reader.read_event_into_async(&mut buf).await {
      Ok(Event::Start(tag)) => match tag.name().as_ref() {
        b"LawNum" => is_law_num_mode = true,
        b"Part" => {
          chapter_num = Chapter {
            part: {
              match chapter_num.part {
                Some(n) => Some(n + 1),
                None => Some(1),
              }
            },
            chapter: None,
            section: None,
            subsection: None,
            division: None,
            article: chapter_num.article,
            paragraph: None,
            item: None,
            sub_item: None,
            suppl_provision_title: chapter_num.suppl_provision_title,
          };
        }
        b"Chapter" => {
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: {
              match chapter_num.chapter {
                Some(n) => Some(n + 1),
                None => Some(1),
              }
            },
            section: None,
            subsection: None,
            division: None,
            article: chapter_num.article,
            paragraph: None,
            item: None,
            sub_item: None,
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"Section" => {
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: {
              match chapter_num.section {
                Some(n) => Some(n + 1),
                None => Some(1),
              }
            },
            subsection: None,
            division: None,
            article: chapter_num.article,
            paragraph: None,
            item: None,
            sub_item: None,
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"Subsection" => {
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: {
              match chapter_num.subsection {
                Some(n) => Some(n + 1),
                None => Some(1),
              }
            },
            division: None,
            article: chapter_num.article,
            paragraph: None,
            item: None,
            sub_item: None,
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"Division" => {
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: {
              match chapter_num.division {
                Some(n) => Some(n + 1),
                None => Some(1),
              }
            },
            article: chapter_num.article,
            paragraph: None,
            item: None,
            sub_item: None,
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"Article" => {
          let article_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: article_num_str,
            paragraph: None,
            item: None,
            sub_item: None,
            suppl_provision_title: chapter_num.suppl_provision_title,
          };
        }
        b"Paragraph" => {
          let paragraph_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: Some(paragraph_num_str),
            item: None,
            sub_item: None,
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"Item" => {
          let item_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: chapter_num.paragraph,
            item: Some(item_num_str),
            sub_item: None,
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"SubItem1" => {
          let sub_item_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: chapter_num.paragraph,
            item: chapter_num.item,
            sub_item: Some((1, sub_item_num_str)),
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"SubItem2" => {
          let sub_item_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: chapter_num.paragraph,
            item: chapter_num.item,
            sub_item: Some((2, sub_item_num_str)),
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"SubItem3" => {
          let sub_item_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: chapter_num.paragraph,
            item: chapter_num.item,
            sub_item: Some((3, sub_item_num_str)),
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"SubItem4" => {
          let sub_item_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: chapter_num.paragraph,
            item: chapter_num.item,
            sub_item: Some((4, sub_item_num_str)),
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"SubItem5" => {
          let sub_item_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: chapter_num.paragraph,
            item: chapter_num.item,
            sub_item: Some((5, sub_item_num_str)),
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"SubItem6" => {
          let sub_item_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: chapter_num.paragraph,
            item: chapter_num.item,
            sub_item: Some((6, sub_item_num_str)),
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        b"SubItem7" => {
          let sub_item_num_str = find_attribute_str(&mut tag.attributes(), "Num", &chapter_num)?;
          chapter_num = Chapter {
            part: chapter_num.part,
            chapter: chapter_num.chapter,
            section: chapter_num.section,
            subsection: chapter_num.subsection,
            division: chapter_num.division,
            article: chapter_num.article,
            paragraph: chapter_num.paragraph,
            item: chapter_num.item,
            sub_item: Some((7, sub_item_num_str)),
            suppl_provision_title: chapter_num.suppl_provision_title,
          }
        }
        // 附則
        b"SupplProvision" => {
          chapter_num = Chapter {
            part: None,
            chapter: None,
            section: None,
            subsection: None,
            division: None,
            article: String::new(),
            paragraph: None,
            item: None,
            sub_item: None,
            suppl_provision_title: find_attribute_str(
              &mut tag.attributes(),
              "AmendLawNum",
              &chapter_num,
            )
            .ok(),
          }
        }
        _ => (),
      },
      Ok(Event::End(tag)) => {
        if let b"LawNum" = tag.name().as_ref() {
          is_law_num_mode = false
        }
      }
      Ok(Event::Text(text)) => {
        if is_law_num_mode {
          let b = text.into_inner();
          law_num = encoding::decode(&b, utf8)
            .map_err(|_| GetArticleError::Encoding(chapter_num.clone(), b.to_vec()))?
            .to_string();
        } else {
          let b = text.into_inner();
          let text_str = encoding::decode(&b, utf8)
            .map_err(|_| GetArticleError::Encoding(chapter_num.clone(), b.to_vec()))?
            .to_string();
          if f(&text_str) {
            lst.push(chapter_num.clone())
          }
        }
      }
      Ok(Event::Eof) => break,
      Err(e) => panic!("法令名APIの結果のXMLの解析中のエラー: {e}"),
      _ => (),
    }
  }
  lst.sort();
  lst.dedup();
  Ok(LawParagraph {
    num: law_num,
    chapter_data: lst,
  })
}

