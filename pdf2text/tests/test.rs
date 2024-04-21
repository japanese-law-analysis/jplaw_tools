use pdf2text::*;

#[test]
fn check_1() {
  let path = "tests/092891_hanrei.pdf";
  let s = pdf2text(path).unwrap();
  assert!(s.contains("他方、被告人は、当公判廷において反省の態度を示すだけでなく、保釈後、本"));
}

#[test]
fn check_clean_up() {
  let path = "tests/092900_hanrei.pdf";
  let s = pdf2text(path).unwrap();
  let s = clean_up(&s);
  assert!(s.contains(
    "３  被告のため、この判決に対する上告及び上告受理申立てのための付加\n期間を３０日と定める。"
  ));
  assert!(s.contains("本件は、商標登録無効審判請求に係る不成立審決の取消訴訟である。争点は、①\n別紙登録商標目録記載の登録商標（以下「本件商標」という。）が商標法４条１項"));
}
