//! 法律そのものに関する情報

use japanese_law_xml_schema::law::Era;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

/// 日付（元号）
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Date {
  pub era: Era,
  pub year: usize,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub month: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub day: Option<usize>,
}

impl Date {
  pub fn new(era: Era, year: usize, month: Option<usize>, day: Option<usize>) -> Self {
    Self {
      era,
      year,
      month,
      day,
    }
  }

  /// 西暦の取得
  pub fn get_ad(&self) -> usize {
    use Era::*;
    match self.era {
      Meiji => 1867 + self.year,
      Taisho => 1911 + self.year,
      Showa => 1925 + self.year,
      Heisei => 1988 + self.year,
      Reiwa => 2018 + self.year,
    }
  }

  /// 西暦からの構築
  pub fn gen_from_ad(year: usize, month: usize, day: usize) -> Self {
    use Era::*;
    let t = year * 10000 + month * 100 + day;
    let (era, year) = if (18681023..=19120729).contains(&t) {
      (Meiji, year - 1867)
    } else if (19120730..=19261224).contains(&t) {
      (Taisho, year - 1920)
    } else if (19261225..=19890107).contains(&t) {
      (Showa, year - 1925)
    } else if (19890108..=20190430).contains(&t) {
      (Heisei, year - 1988)
    } else if 20190501 <= t {
      (Reiwa, year - 2018)
    } else {
      unreachable!()
    };
    Self {
      era,
      year,
      month: Some(month),
      day: Some(day),
    }
  }
}

impl PartialOrd for Date {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Date {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let s = self.get_ad() * 10000 + self.month.unwrap_or(0) * 100 + self.day.unwrap_or(0);
    let o = other.get_ad() * 10000 + other.month.unwrap_or(0) * 100 + other.day.unwrap_or(0);
    s.cmp(&o)
  }
}

/// 法律の立法の種類
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum RippouType {
  /// 閣法
  Kakuhou,
  /// 衆議院議員立法
  Syuin,
  /// 参議院議員立法
  Sanin,
}

/// 法律の効力の種類
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LawEfficacy {
  /// 政令
  CabinetOrder,
  /// 法律
  Law,
}

/// 府・省に共通化させる
trait MinistryContents: Sized {
  fn to_int(&self) -> usize;
  fn from_int(n: usize) -> Option<Self>;
}

/// M1時（1869年7月8日〜1943年10月31日）での府・省
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum M1Ministry {
  /// 閣令
  CabinetOrder,
  /// 宮内省令
  ImperialHouseholdOrdinance,
  /// 大東亜省令
  GreaterEastAsiaMinisterialOrdinance,
  /// 内務省令
  MinistryOfTheInteriorOrdinance,
  /// 司法省令
  MinistryOfJusticeOrdinance,
  /// 外務省令
  MinistryOfForeignAffairsOrdinance,
  /// 大蔵省令
  MinistryOfFinanceOrdinance,
  /// 文部省令
  MinistryOfEducationOrdinance,
  /// 厚生省令
  MinistryOfHealthAndWelfareOrdinance,
  /// 農商務省令
  MinistryOfAgricultureAndCommerceOrdinance,
  /// 商工省令
  MinistryOfCommerceAndIndustryOrdinance,
  /// 鉄道省令
  RailwayMinisterialOrdinance,
  /// 逓信省令
  MinistryOfCommunicationsOrdinance,
  /// 陸軍省令（甲）
  MinistryOfTheArmyOrdinanceA,
  ///海軍省令
  NavyMinisterialOrdinance,
  /// 陸軍省令（乙）
  MinistryOfTheArmyOrdinanceB,
  /// 農林省令
  MinistryOfAgricultureAndForestryOrdinance,
  /// 拓殖務省令
  MinistryOfLandDevelopmentOrdinanceA,
  /// 拓務省令
  MinistryOfLandDevelopmentOrdinanceB,
  /// 農商務省令臨
  MinistryOfAgricultureAndCommerceOrdinanceTemporary,
  /// 司法省令（丙）
  MinistryOfJusticeOrdinanceHei,
}

impl MinistryContents for M1Ministry {
  fn to_int(&self) -> usize {
    use M1Ministry::*;
    match self {
      CabinetOrder => 1,
      ImperialHouseholdOrdinance => 2,
      GreaterEastAsiaMinisterialOrdinance => 3,
      MinistryOfTheInteriorOrdinance => 4,
      MinistryOfJusticeOrdinance => 5,
      MinistryOfForeignAffairsOrdinance => 6,
      MinistryOfFinanceOrdinance => 7,
      MinistryOfEducationOrdinance => 8,
      MinistryOfHealthAndWelfareOrdinance => 9,
      MinistryOfAgricultureAndCommerceOrdinance => 10,
      MinistryOfCommerceAndIndustryOrdinance => 11,
      RailwayMinisterialOrdinance => 12,
      MinistryOfCommunicationsOrdinance => 13,
      MinistryOfTheArmyOrdinanceA => 14,
      NavyMinisterialOrdinance => 15,
      MinistryOfTheArmyOrdinanceB => 16,
      MinistryOfAgricultureAndForestryOrdinance => 17,
      MinistryOfLandDevelopmentOrdinanceA => 18,
      MinistryOfLandDevelopmentOrdinanceB => 19,
      MinistryOfAgricultureAndCommerceOrdinanceTemporary => 20,
      MinistryOfJusticeOrdinanceHei => 21,
    }
  }

  fn from_int(n: usize) -> Option<Self> {
    use M1Ministry::*;
    match n {
      1 => Some(CabinetOrder),
      2 => Some(ImperialHouseholdOrdinance),
      3 => Some(GreaterEastAsiaMinisterialOrdinance),
      4 => Some(MinistryOfTheInteriorOrdinance),
      5 => Some(MinistryOfJusticeOrdinance),
      6 => Some(MinistryOfForeignAffairsOrdinance),
      7 => Some(MinistryOfFinanceOrdinance),
      8 => Some(MinistryOfEducationOrdinance),
      9 => Some(MinistryOfHealthAndWelfareOrdinance),
      10 => Some(MinistryOfAgricultureAndCommerceOrdinance),
      11 => Some(MinistryOfCommerceAndIndustryOrdinance),
      12 => Some(RailwayMinisterialOrdinance),
      13 => Some(MinistryOfCommunicationsOrdinance),
      14 => Some(MinistryOfTheArmyOrdinanceA),
      15 => Some(NavyMinisterialOrdinance),
      16 => Some(MinistryOfTheArmyOrdinanceB),
      17 => Some(MinistryOfAgricultureAndForestryOrdinance),
      18 => Some(MinistryOfLandDevelopmentOrdinanceA),
      19 => Some(MinistryOfLandDevelopmentOrdinanceB),
      20 => Some(MinistryOfAgricultureAndCommerceOrdinanceTemporary),
      21 => Some(MinistryOfJusticeOrdinanceHei),
      _ => None,
    }
  }
}

/// M2時（1943年11月1日〜1945年11月31日）での府・省
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum M2Ministry {
  /// 閣令
  CabinetOrder,
  /// 宮内省令
  ImperialHouseholdOrdinance,
  /// 大東亜省令
  GreaterEastAsiaMinisterialOrdinance,
  /// 内務省令
  MinistryOfTheInteriorOrdinance,
  /// 司法省令
  MinistryOfJusticeOrdinance,
  /// 外務省令
  MinistryOfForeignAffairsOrdinance,
  /// 大蔵省令
  MinistryOfFinanceOrdinance,
  /// 文部省令
  MinistryOfEducationOrdinance,
  /// 厚生省令
  MinistryOfHealthAndWelfareOrdinance,
  /// 農商務省令
  MinistryOfAgricultureAndCommerceOrdinance,
  /// 商工省令
  MinistryOfCommerceAndIndustryOrdinance,
  /// 運輸省令
  MinistryOfTransportOrdinance,
  /// 運輸通信省令
  MinistryOfTransportAndCommunicationsOrdinance,
  /// 陸軍省令（甲）
  MinistryOfTheArmyOrdinanceA,
  ///海軍省令
  NavyMinisterialOrdinance,
  /// 軍需省令
  OrdinanceOfTheMinistryOfMunitions,
  /// 農林省令
  MinistryOfAgricultureAndForestryOrdinance,
}

impl MinistryContents for M2Ministry {
  fn to_int(&self) -> usize {
    use M2Ministry::*;
    match self {
      CabinetOrder => 1,
      ImperialHouseholdOrdinance => 2,
      GreaterEastAsiaMinisterialOrdinance => 3,
      MinistryOfTheInteriorOrdinance => 4,
      MinistryOfJusticeOrdinance => 5,
      MinistryOfForeignAffairsOrdinance => 6,
      MinistryOfFinanceOrdinance => 7,
      MinistryOfEducationOrdinance => 8,
      MinistryOfHealthAndWelfareOrdinance => 9,
      MinistryOfAgricultureAndCommerceOrdinance => 10,
      MinistryOfCommerceAndIndustryOrdinance => 11,
      MinistryOfTransportOrdinance => 12,
      MinistryOfTransportAndCommunicationsOrdinance => 13,
      MinistryOfTheArmyOrdinanceA => 14,
      NavyMinisterialOrdinance => 15,
      OrdinanceOfTheMinistryOfMunitions => 16,
      MinistryOfAgricultureAndForestryOrdinance => 17,
    }
  }

  fn from_int(n: usize) -> Option<Self> {
    use M2Ministry::*;
    match n {
      1 => Some(CabinetOrder),
      2 => Some(ImperialHouseholdOrdinance),
      3 => Some(GreaterEastAsiaMinisterialOrdinance),
      4 => Some(MinistryOfTheInteriorOrdinance),
      5 => Some(MinistryOfJusticeOrdinance),
      6 => Some(MinistryOfForeignAffairsOrdinance),
      7 => Some(MinistryOfFinanceOrdinance),
      8 => Some(MinistryOfEducationOrdinance),
      9 => Some(MinistryOfHealthAndWelfareOrdinance),
      10 => Some(MinistryOfAgricultureAndCommerceOrdinance),
      11 => Some(MinistryOfCommerceAndIndustryOrdinance),
      12 => Some(MinistryOfTransportOrdinance),
      13 => Some(MinistryOfTransportAndCommunicationsOrdinance),
      14 => Some(MinistryOfTheArmyOrdinanceA),
      15 => Some(NavyMinisterialOrdinance),
      16 => Some(OrdinanceOfTheMinistryOfMunitions),
      17 => Some(MinistryOfAgricultureAndForestryOrdinance),
      _ => None,
    }
  }
}

/// M3時（1945年12月1日〜1947年5月2日）での府・省
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum M3Ministry {
  /// 閣令
  CabinetOrder,
  /// 宮内省令
  ImperialHouseholdOrdinance,
  /// 経済安定本部令
  EconomicStabilityHeadquartersOrdinance,
  /// 内務省令
  MinistryOfTheInteriorOrdinance,
  /// 司法省令
  MinistryOfJusticeOrdinance,
  /// 外務省令
  MinistryOfForeignAffairsOrdinance,
  /// 大蔵省令
  MinistryOfFinanceOrdinance,
  /// 文部省令
  MinistryOfEducationOrdinance,
  /// 厚生省令
  MinistryOfHealthAndWelfareOrdinance,
  /// 農林省令
  MinistryOfAgricultureAndForestryOrdinance,
  /// 商工省令
  MinistryOfCommerceAndIndustryOrdinance,
  /// 運輸省令
  MinistryOfTransportOrdinance,
  /// 逓信省令
  MinistryOfCommunicationsOrdinance,
  /// 第一復員省令
  FirstMinisterialOrdinanceForDemobilization,
  /// 第二復員省令
  SecondMinisterialOrdinanceForDemobilization,
  /// 物価庁令
  PriceAgencyOrdinance,
  /// 中央労働委員会規則
  CentralLaborRelationsCommissionRules,
}

impl MinistryContents for M3Ministry {
  fn to_int(&self) -> usize {
    use M3Ministry::*;
    match self {
      CabinetOrder => 1,
      ImperialHouseholdOrdinance => 2,
      EconomicStabilityHeadquartersOrdinance => 3,
      MinistryOfTheInteriorOrdinance => 4,
      MinistryOfJusticeOrdinance => 5,
      MinistryOfForeignAffairsOrdinance => 6,
      MinistryOfFinanceOrdinance => 7,
      MinistryOfEducationOrdinance => 8,
      MinistryOfHealthAndWelfareOrdinance => 9,
      MinistryOfAgricultureAndForestryOrdinance => 10,
      MinistryOfCommerceAndIndustryOrdinance => 11,
      MinistryOfTransportOrdinance => 12,
      MinistryOfCommunicationsOrdinance => 13,
      FirstMinisterialOrdinanceForDemobilization => 14,
      SecondMinisterialOrdinanceForDemobilization => 15,
      PriceAgencyOrdinance => 16,
      CentralLaborRelationsCommissionRules => 21,
    }
  }

  fn from_int(n: usize) -> Option<Self> {
    use M3Ministry::*;
    match n {
      1 => Some(CabinetOrder),
      2 => Some(ImperialHouseholdOrdinance),
      3 => Some(EconomicStabilityHeadquartersOrdinance),
      4 => Some(MinistryOfTheInteriorOrdinance),
      5 => Some(MinistryOfJusticeOrdinance),
      6 => Some(MinistryOfForeignAffairsOrdinance),
      7 => Some(MinistryOfFinanceOrdinance),
      8 => Some(MinistryOfEducationOrdinance),
      9 => Some(MinistryOfHealthAndWelfareOrdinance),
      10 => Some(MinistryOfAgricultureAndForestryOrdinance),
      11 => Some(MinistryOfCommerceAndIndustryOrdinance),
      12 => Some(MinistryOfTransportOrdinance),
      13 => Some(MinistryOfCommunicationsOrdinance),
      14 => Some(FirstMinisterialOrdinanceForDemobilization),
      15 => Some(SecondMinisterialOrdinanceForDemobilization),
      16 => Some(PriceAgencyOrdinance),
      21 => Some(CentralLaborRelationsCommissionRules),
      _ => None,
    }
  }
}

/// M4時（1947年5月3日〜1949年5月31日）での府・省
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum M4Ministry {
  /// 法務庁令
  LegalAffairsAgencyOrdinance,
  /// 総理庁令
  PrimeMinistersOfficeOrdinance,
  /// 経済安定本部令
  EconomicStabilityHeadquartersOrdinance,
  /// 内務省令
  MinistryOfTheInteriorOrdinance,
  /// 司法省令
  MinistryOfJusticeOrdinance,
  /// 外務省令
  MinistryOfForeignAffairsOrdinance,
  /// 大蔵省令
  MinistryOfFinanceOrdinance,
  /// 文部省令
  MinistryOfEducationOrdinance,
  /// 厚生省令
  MinistryOfHealthAndWelfareOrdinance,
  /// 農林省令
  MinistryOfAgricultureAndForestryOrdinance,
  /// 通商産業省令
  MinistryOfInternationalTradeAndIndustryOrdinance,
  /// 運輸省令
  MinistryOfTransportOrdinance,
  /// 逓信省令
  MinistryOfCommunicationsOrdinance,
  /// 労働省令
  MinistryOfLaborOrdinance,
  /// 建設省令
  MinistryOfConstructionOrdinance,
  /// 物価庁令
  PriceAgencyOrdinance,
  /// 商工省令
  MinistryOfCommerceAndIndustryOrdinance,
  /// 中央労働委員会規則
  CentralLaborRelationsCommissionRules,
  /// 公正取引委員会規則
  FairTradeCommissionRules,
  /// 国家公安委員会規則
  NationalPublicSafetyCommissionRegulations,
}

impl MinistryContents for M4Ministry {
  fn to_int(&self) -> usize {
    use M4Ministry::*;
    match self {
      LegalAffairsAgencyOrdinance => 1,
      PrimeMinistersOfficeOrdinance => 2,
      EconomicStabilityHeadquartersOrdinance => 3,
      MinistryOfTheInteriorOrdinance => 4,
      MinistryOfJusticeOrdinance => 5,
      MinistryOfForeignAffairsOrdinance => 6,
      MinistryOfFinanceOrdinance => 7,
      MinistryOfEducationOrdinance => 8,
      MinistryOfHealthAndWelfareOrdinance => 9,
      MinistryOfAgricultureAndForestryOrdinance => 10,
      MinistryOfInternationalTradeAndIndustryOrdinance => 11,
      MinistryOfTransportOrdinance => 12,
      MinistryOfCommunicationsOrdinance => 13,
      MinistryOfLaborOrdinance => 14,
      MinistryOfConstructionOrdinance => 15,
      PriceAgencyOrdinance => 16,
      MinistryOfCommerceAndIndustryOrdinance => 17,
      CentralLaborRelationsCommissionRules => 21,
      FairTradeCommissionRules => 22,
      NationalPublicSafetyCommissionRegulations => 23,
    }
  }

  fn from_int(n: usize) -> Option<Self> {
    use M4Ministry::*;
    match n {
      1 => Some(LegalAffairsAgencyOrdinance),
      2 => Some(PrimeMinistersOfficeOrdinance),
      3 => Some(EconomicStabilityHeadquartersOrdinance),
      4 => Some(MinistryOfTheInteriorOrdinance),
      5 => Some(MinistryOfJusticeOrdinance),
      6 => Some(MinistryOfForeignAffairsOrdinance),
      7 => Some(MinistryOfFinanceOrdinance),
      8 => Some(MinistryOfEducationOrdinance),
      9 => Some(MinistryOfHealthAndWelfareOrdinance),
      10 => Some(MinistryOfAgricultureAndForestryOrdinance),
      11 => Some(MinistryOfInternationalTradeAndIndustryOrdinance),
      12 => Some(MinistryOfTransportOrdinance),
      13 => Some(MinistryOfCommunicationsOrdinance),
      14 => Some(MinistryOfLaborOrdinance),
      15 => Some(MinistryOfConstructionOrdinance),
      16 => Some(PriceAgencyOrdinance),
      17 => Some(MinistryOfCommerceAndIndustryOrdinance),
      21 => Some(CentralLaborRelationsCommissionRules),
      22 => Some(FairTradeCommissionRules),
      23 => Some(NationalPublicSafetyCommissionRegulations),
      _ => None,
    }
  }
}

/// M5時（1949年6月1日〜2001年1月15日）での府・省
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum M5Ministry {
  /// 法務庁令
  LegalAffairsAgencyOrdinance,
  /// 総理庁令
  PrimeMinistersOfficeOrdinance,
  /// 経済安定本部令
  EconomicStabilityHeadquartersOrdinance,
  /// 自治省令
  MinistryOfHomeAffairsOrdinance,
  /// 法務省令
  MinistryOfJusticeOrdinance,
  /// 外務省令
  MinistryOfForeignAffairsOrdinance,
  /// 大蔵省令
  MinistryOfFinanceOrdinance,
  /// 文部省令
  MinistryOfEducationOrdinance,
  /// 厚生省令
  MinistryOfHealthAndWelfareOrdinance,
  /// 農林水産省令
  MinistryOfAgricultureAndForestryAndFisheriesOrdinance,
  /// 通商産業省令
  MinistryOfInternationalTradeAndIndustryOrdinance,
  /// 運輸省令
  MinistryOfTransportOrdinance,
  /// 郵政省令
  MinistryOfPostsAndTelecommunicationsOrdinance,
  /// 労働省令
  MinistryOfLaborOrdinance,
  /// 建設省令
  MinistryOfConstructionOrdinance,
  /// 物価庁令
  PriceAgencyOrdinance,
  /// 農林省令
  MinistryOfAgricultureAndForestryOrdinance,
  /// 電気通信省令
  TelecommunicationsMinisterialOrdinance,
  /// 中央省庁等改革推進本部令
  CentralMinistriesAndAgenciesReformPromotionHeadquartersOrdinance,
  /// 電波監理委員会規則
  RadioRegulatoryCommissionRules,
  /// 中央労働委員会規則
  CentralLaborRelationsCommissionRules,
  /// 公正取引委員会規則
  FairTradeCommissionRules,
  /// 国家公安委員会規則
  NationalPublicSafetyCommissionRegulations,
  /// 公害等調整委員会規則
  PollutionAdjustmentCommitteeRules,
  /// 公安審査委員会規則
  PublicSafetyReviewCommitteeRules,
}

impl MinistryContents for M5Ministry {
  fn to_int(&self) -> usize {
    use M5Ministry::*;
    match self {
      LegalAffairsAgencyOrdinance => 1,
      PrimeMinistersOfficeOrdinance => 2,
      EconomicStabilityHeadquartersOrdinance => 3,
      MinistryOfHomeAffairsOrdinance => 4,
      MinistryOfJusticeOrdinance => 5,
      MinistryOfForeignAffairsOrdinance => 6,
      MinistryOfFinanceOrdinance => 7,
      MinistryOfEducationOrdinance => 8,
      MinistryOfHealthAndWelfareOrdinance => 9,
      MinistryOfAgricultureAndForestryAndFisheriesOrdinance => 10,
      MinistryOfInternationalTradeAndIndustryOrdinance => 11,
      MinistryOfTransportOrdinance => 12,
      MinistryOfPostsAndTelecommunicationsOrdinance => 13,
      MinistryOfLaborOrdinance => 14,
      MinistryOfConstructionOrdinance => 15,
      PriceAgencyOrdinance => 16,
      MinistryOfAgricultureAndForestryOrdinance => 17,
      TelecommunicationsMinisterialOrdinance => 18,
      CentralMinistriesAndAgenciesReformPromotionHeadquartersOrdinance => 19,
      RadioRegulatoryCommissionRules => 20,
      CentralLaborRelationsCommissionRules => 21,
      FairTradeCommissionRules => 22,
      NationalPublicSafetyCommissionRegulations => 23,
      PollutionAdjustmentCommitteeRules => 24,
      PublicSafetyReviewCommitteeRules => 25,
    }
  }

  fn from_int(n: usize) -> Option<Self> {
    use M5Ministry::*;
    match n {
      1 => Some(LegalAffairsAgencyOrdinance),
      2 => Some(PrimeMinistersOfficeOrdinance),
      3 => Some(EconomicStabilityHeadquartersOrdinance),
      4 => Some(MinistryOfHomeAffairsOrdinance),
      5 => Some(MinistryOfJusticeOrdinance),
      6 => Some(MinistryOfForeignAffairsOrdinance),
      7 => Some(MinistryOfFinanceOrdinance),
      8 => Some(MinistryOfEducationOrdinance),
      9 => Some(MinistryOfHealthAndWelfareOrdinance),
      10 => Some(MinistryOfAgricultureAndForestryAndFisheriesOrdinance),
      11 => Some(MinistryOfInternationalTradeAndIndustryOrdinance),
      12 => Some(MinistryOfTransportOrdinance),
      13 => Some(MinistryOfPostsAndTelecommunicationsOrdinance),
      14 => Some(MinistryOfLaborOrdinance),
      15 => Some(MinistryOfConstructionOrdinance),
      16 => Some(PriceAgencyOrdinance),
      17 => Some(MinistryOfAgricultureAndForestryOrdinance),
      18 => Some(TelecommunicationsMinisterialOrdinance),
      19 => Some(CentralMinistriesAndAgenciesReformPromotionHeadquartersOrdinance),
      20 => Some(RadioRegulatoryCommissionRules),
      21 => Some(CentralLaborRelationsCommissionRules),
      22 => Some(FairTradeCommissionRules),
      23 => Some(NationalPublicSafetyCommissionRegulations),
      24 => Some(PollutionAdjustmentCommitteeRules),
      25 => Some(PublicSafetyReviewCommitteeRules),
      _ => None,
    }
  }
}

/// M6時（2001年1月16日〜）での府・省
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum M6Ministry {
  /// 内閣官房令
  CabinetSecretariatOrdinance,
  /// 総理庁令
  PrimeMinistersOfficeOrdinance,
  /// 復興庁令
  ReconstructionAgencyOrdinance,
  /// 自治省令
  MinistryOfHomeAffairsOrdinance,
  /// 法務省令
  MinistryOfJusticeOrdinance,
  /// 外務省令
  MinistryOfForeignAffairsOrdinance,
  /// 財務省令
  MinistryOfFinanceOrdinance,
  /// 文部科学省令
  MinistryOfEducationAndCultureAndSportsAndScienceAndTechnologyOrdinance,
  /// 厚生労働省令
  MinistryOfHealthAndLaborAndWelfareOrdinance,
  /// 農林水産省令
  MinistryOfAgricultureAndForestryAndFisheriesOrdinance,
  /// 経済産業省令
  MinistryOfEconomyAndTradeAndIndustryOrdinance,
  /// 国土交通省令
  MinistryOfLandAndInfrastructureAndTransportAndTourismOrdinance,
  /// 環境省令
  MinistryOfTheEnvironmentOrdinance,
  /// 防衛省令
  MinistryOfDefenseOrdinance,
  /// デジタル庁令
  DigitalAgencyOrdinance,
  /// 特定個人情報保護委員会規則
  SpecificPersonalInformationProtectionCommissionRules,
  /// 運輸安全委員会規則
  JapanTransportSafetyBoardRegulations,
  /// 原子力規制委員会規則
  NuclearRegulationAuthorityRegulations,
  /// 中央労働委員会規則
  CentralLaborRelationsCommissionRules,
  /// 公正取引委員会規則
  FairTradeCommissionRules,
  /// 国家公安委員会規則
  NationalPublicSafetyCommissionRegulations,
  /// 公害等調整委員会規則
  PollutionAdjustmentCommitteeRules,
  /// 公安審査委員会規則
  PublicSafetyReviewCommitteeRules,
  /// カジノ管理委員会規則
  CasinoManagementCommitteeRules,
}

impl MinistryContents for M6Ministry {
  fn to_int(&self) -> usize {
    use M6Ministry::*;
    match self {
      CabinetSecretariatOrdinance => 1,
      PrimeMinistersOfficeOrdinance => 2,
      ReconstructionAgencyOrdinance => 3,
      MinistryOfHomeAffairsOrdinance => 4,
      MinistryOfJusticeOrdinance => 5,
      MinistryOfForeignAffairsOrdinance => 6,
      MinistryOfFinanceOrdinance => 7,
      MinistryOfEducationAndCultureAndSportsAndScienceAndTechnologyOrdinance => 8,
      MinistryOfHealthAndLaborAndWelfareOrdinance => 9,
      MinistryOfAgricultureAndForestryAndFisheriesOrdinance => 10,
      MinistryOfEconomyAndTradeAndIndustryOrdinance => 11,
      MinistryOfLandAndInfrastructureAndTransportAndTourismOrdinance => 12,
      MinistryOfTheEnvironmentOrdinance => 13,
      MinistryOfDefenseOrdinance => 14,
      DigitalAgencyOrdinance => 15,
      SpecificPersonalInformationProtectionCommissionRules => 18,
      JapanTransportSafetyBoardRegulations => 19,
      NuclearRegulationAuthorityRegulations => 20,
      CentralLaborRelationsCommissionRules => 21,
      FairTradeCommissionRules => 22,
      NationalPublicSafetyCommissionRegulations => 23,
      PollutionAdjustmentCommitteeRules => 24,
      PublicSafetyReviewCommitteeRules => 25,
      CasinoManagementCommitteeRules => 26,
    }
  }

  fn from_int(n: usize) -> Option<Self> {
    use M6Ministry::*;
    match n {
      1 => Some(CabinetSecretariatOrdinance),
      2 => Some(PrimeMinistersOfficeOrdinance),
      3 => Some(ReconstructionAgencyOrdinance),
      4 => Some(MinistryOfHomeAffairsOrdinance),
      5 => Some(MinistryOfJusticeOrdinance),
      6 => Some(MinistryOfForeignAffairsOrdinance),
      7 => Some(MinistryOfFinanceOrdinance),
      8 => Some(MinistryOfEducationAndCultureAndSportsAndScienceAndTechnologyOrdinance),
      9 => Some(MinistryOfHealthAndLaborAndWelfareOrdinance),
      10 => Some(MinistryOfAgricultureAndForestryAndFisheriesOrdinance),
      11 => Some(MinistryOfEconomyAndTradeAndIndustryOrdinance),
      12 => Some(MinistryOfLandAndInfrastructureAndTransportAndTourismOrdinance),
      13 => Some(MinistryOfTheEnvironmentOrdinance),
      14 => Some(MinistryOfDefenseOrdinance),
      15 => Some(DigitalAgencyOrdinance),
      18 => Some(SpecificPersonalInformationProtectionCommissionRules),
      19 => Some(JapanTransportSafetyBoardRegulations),
      20 => Some(NuclearRegulationAuthorityRegulations),
      21 => Some(CentralLaborRelationsCommissionRules),
      22 => Some(FairTradeCommissionRules),
      23 => Some(NationalPublicSafetyCommissionRegulations),
      24 => Some(PollutionAdjustmentCommitteeRules),
      25 => Some(PublicSafetyReviewCommitteeRules),
      26 => Some(CasinoManagementCommitteeRules),
      _ => None,
    }
  }
}

/// 府・省
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ministry {
  /// 1869年7月8日〜1943年10月31日
  M1(Vec<M1Ministry>),
  /// 1943年11月1日〜1945年11月31日
  M2(Vec<M2Ministry>),
  /// 1945年12月1日〜1947年5月2日
  M3(Vec<M3Ministry>),
  /// 1947年5月3日〜1949年5月31日
  M4(Vec<M4Ministry>),
  /// 1949年6月1日〜2001年1月15日
  M5(Vec<M5Ministry>),
  /// 2001年1月16日〜
  M6(Vec<M6Ministry>),
}

fn ministry_list_to_usize<T: MinistryContents>(l: &[T]) -> usize {
  let mut n = 0;
  for u in l.iter().map(|v| v.to_int() as u32) {
    n |= 2_u32.pow(u - 1);
  }
  n as usize
}

fn list_from_str<T: MinistryContents + std::fmt::Debug>(byte_s: &str) -> Result<Vec<T>, ()> {
  let chars = byte_s.chars();
  let mut v = Vec::new();
  for (i, c) in chars.enumerate() {
    if c == '1' {
      let n = 28 - i;
      if let Some(t) = T::from_int(n) {
        v.push(t);
      } else {
        return Err(());
      }
    } else if c == '0' {
    } else {
      return Err(());
    }
  }
  Ok(v)
}

/// 機関名
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Institution {
  /// 会計検査院
  BoardOfAudit,
  /// 海上保安庁
  CoastGuard,
  /// 日本学術会議
  ScienceCouncilOfJapan,
  /// 土地調整委員会
  LandAdjustmentCommittee,
  /// 金融再生委員会
  FinancialReconstructionCommittee,
  /// 首都圏整備委員会
  MetropolitanAreaDevelopmentCommittee,
  /// 地方財政委員会
  LocalFinanceCommittee,
  /// 司法試験管理委員会
  BarExaminationManagementCommittee,
  /// 公認会計士管理委員会
  CertifiedPublicAccountantManagementCommittee,
  /// 外資委員会
  ForeignInvestmentCommittee,
  /// 文化財保護委員会
  CulturalPropertiesProtectionCommittee,
  /// 日本ユネスコ国内委員会
  JapaneseNationalCommissionForUNESCO,
  /// 最高裁判所
  SupremeCourt,
  /// 衆議院
  HouseOfRepresentatives,
  /// 参議院
  HouseOfCouncilors,
  /// 船員中央労働委員会
  SeafarersCentralLaborCommittee,
  // /// 司法試験管理委員会
  /// 電波監理委員会
  RadioRegulatoryCommission,
  /// カジノ管理委員会
  CasinoManagementCommittee,
}

impl Institution {
  pub(crate) fn to_int(&self) -> usize {
    use Institution::*;
    match self {
      BoardOfAudit => 1,
      CoastGuard => 2,
      ScienceCouncilOfJapan => 3,
      LandAdjustmentCommittee => 4,
      FinancialReconstructionCommittee => 5,
      MetropolitanAreaDevelopmentCommittee => 6,
      LocalFinanceCommittee => 7,
      BarExaminationManagementCommittee => 8,
      CertifiedPublicAccountantManagementCommittee => 9,
      ForeignInvestmentCommittee => 10,
      CulturalPropertiesProtectionCommittee => 11,
      JapaneseNationalCommissionForUNESCO => 12,
      SupremeCourt => 13,
      HouseOfRepresentatives => 14,
      HouseOfCouncilors => 15,
      SeafarersCentralLaborCommittee => 16,
      RadioRegulatoryCommission => 18,
      CasinoManagementCommittee => 19,
    }
  }

  pub(crate) fn from_int(n: usize) -> Option<Self> {
    use Institution::*;
    match n {
      1 => Some(BoardOfAudit),
      2 => Some(CoastGuard),
      3 => Some(ScienceCouncilOfJapan),
      4 => Some(LandAdjustmentCommittee),
      5 => Some(FinancialReconstructionCommittee),
      6 => Some(MetropolitanAreaDevelopmentCommittee),
      7 => Some(LocalFinanceCommittee),
      8 => Some(BarExaminationManagementCommittee),
      9 => Some(CertifiedPublicAccountantManagementCommittee),
      10 => Some(ForeignInvestmentCommittee),
      11 => Some(CulturalPropertiesProtectionCommittee),
      12 => Some(JapaneseNationalCommissionForUNESCO),
      13 => Some(SupremeCourt),
      14 => Some(HouseOfRepresentatives),
      15 => Some(HouseOfCouncilors),
      16 => Some(SeafarersCentralLaborCommittee),
      17 => Some(BarExaminationManagementCommittee),
      18 => Some(RadioRegulatoryCommission),
      19 => Some(CasinoManagementCommittee),
      _ => None,
    }
  }
}

/// 法令IDの詳細 <https://elaws.e-gov.go.jp/file/LawIdNamingConvention.pdf> を参照
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LawIdType {
  /// 憲法
  Constitution,
  /// 法律
  Act { rippou_type: RippouType, num: usize },
  /// 政令
  CabinetOrder { efficacy: LawEfficacy, num: usize },
  /// 勅令
  ImperialOrder { efficacy: LawEfficacy, num: usize },
  /// 太政官布告
  DajokanFukoku { efficacy: LawEfficacy, num: usize },
  /// 太政官達
  DajokanTasshi { efficacy: LawEfficacy, num: usize },
  /// 太政官布達
  DajokanHutatsu { efficacy: LawEfficacy, num: usize },
  /// 府省令
  MinistryOrder { ministry: Ministry, num: usize },
  /// 人事院規則
  Jinjin {
    /// 規則の分類
    kind: usize,
    /// 規則の分類中の連番
    kind_serial_number: usize,
    /// 改正規則の連番
    amendment_serial_number: usize,
  },
  /// 機関の規則
  Regulation {
    institution: Institution,
    num: usize,
  },
  /// 内閣総理大臣決定の行政機関の規則
  PrimeMinisterDecision {
    /// 決定月
    month: usize,
    /// 決定日
    day: usize,
    /// 同一決定日内の連番
    num: usize,
  },
}

impl Display for LawIdType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use LawIdType::*;
    match self {
      Constitution => write!(f, "CONSTITUTION"),
      Act { rippou_type, num } => match &rippou_type {
        RippouType::Kakuhou => write!(f, "AC0000000{num:03}"),
        RippouType::Syuin => write!(f, "AC1000000{num:03}"),
        RippouType::Sanin => write!(f, "AC0100000{num:03}"),
      },
      CabinetOrder { efficacy, num } => match &efficacy {
        LawEfficacy::Law => write!(f, "CO1000000{num:03}"),
        LawEfficacy::CabinetOrder => write!(f, "CO0000000{num:03}"),
      },
      ImperialOrder { efficacy, num } => match &efficacy {
        LawEfficacy::Law => write!(f, "IO1000000{num:03}"),
        LawEfficacy::CabinetOrder => write!(f, "IO0000000{num:03}"),
      },
      DajokanFukoku { efficacy, num } => match &efficacy {
        LawEfficacy::Law => write!(f, "DF1000000{num:03}"),
        LawEfficacy::CabinetOrder => write!(f, "DF0000000{num:03}"),
      },
      DajokanTasshi { efficacy, num } => match &efficacy {
        LawEfficacy::Law => write!(f, "DT1000000{num:03}"),
        LawEfficacy::CabinetOrder => write!(f, "DT0000000{num:03}"),
      },
      DajokanHutatsu { efficacy, num } => match &efficacy {
        LawEfficacy::Law => write!(f, "DH1000000{num:03}"),
        LawEfficacy::CabinetOrder => write!(f, "DH0000000{num:03}"),
      },
      MinistryOrder { ministry, num } => match &ministry {
        Ministry::M1(m) => write!(f, "M1{:X>07}{num:03}", ministry_list_to_usize(m)),
        Ministry::M2(m) => write!(f, "M2{:X>07}{num:03}", ministry_list_to_usize(m)),
        Ministry::M3(m) => write!(f, "M3{:X>07}{num:03}", ministry_list_to_usize(m)),
        Ministry::M4(m) => write!(f, "M4{:X>07}{num:03}", ministry_list_to_usize(m)),
        Ministry::M5(m) => write!(f, "M5{:07X}{num:03}", ministry_list_to_usize(m)),
        Ministry::M6(m) => write!(f, "M6{:X>07}{num:03}", ministry_list_to_usize(m)),
      },
      Jinjin {
        kind,
        kind_serial_number,
        amendment_serial_number,
      } => write!(
        f,
        "RJNJ{kind:02}{kind_serial_number:03}{amendment_serial_number:03}"
      ),
      Regulation { institution, num } => write!(f, "R{:>07}{num:03}", institution.to_int()),
      PrimeMinisterDecision { month, day, num } => write!(f, "RPMD{month:02}{day:02}{num:04}"),
    }
  }
}

impl FromStr for LawIdType {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use LawIdType::*;
    if s == "CONSTITUTION" {
      Ok(Constitution)
    } else if &s[0..=1] == "AC" {
      let rippou_type_s = &s[2..=8].parse::<usize>().map_err(|_| ())?;
      let rippou_type = if *rippou_type_s == 0 {
        RippouType::Kakuhou
      } else if *rippou_type_s == 1000000 {
        RippouType::Syuin
      } else if *rippou_type_s == 100000 {
        RippouType::Sanin
      } else {
        return Err(());
      };
      let num = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(Act { rippou_type, num })
    } else if &s[0..=1] == "CO" {
      let efficacy_s = &s[2..=8].parse::<usize>().map_err(|_| ())?;
      let efficacy = if *efficacy_s == 0 {
        LawEfficacy::CabinetOrder
      } else if *efficacy_s == 1000000 {
        LawEfficacy::Law
      } else {
        return Err(());
      };
      let num = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(CabinetOrder { efficacy, num })
    } else if &s[0..=1] == "IO" {
      let efficacy_s = &s[2..=8].parse::<usize>().map_err(|_| ())?;
      let efficacy = if *efficacy_s == 0 {
        LawEfficacy::CabinetOrder
      } else if *efficacy_s == 1000000 {
        LawEfficacy::Law
      } else {
        return Err(());
      };
      let num = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(ImperialOrder { efficacy, num })
    } else if &s[0..=1] == "DF" {
      let efficacy_s = &s[2..=8].parse::<usize>().map_err(|_| ())?;
      let efficacy = if *efficacy_s == 0 {
        LawEfficacy::CabinetOrder
      } else if *efficacy_s == 1000000 {
        LawEfficacy::Law
      } else {
        return Err(());
      };
      let num = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(DajokanFukoku { efficacy, num })
    } else if &s[0..=1] == "DT" {
      let efficacy_s = &s[2..=8].parse::<usize>().map_err(|_| ())?;
      let efficacy = if *efficacy_s == 0 {
        LawEfficacy::CabinetOrder
      } else if *efficacy_s == 1000000 {
        LawEfficacy::Law
      } else {
        return Err(());
      };
      let num = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(DajokanTasshi { efficacy, num })
    } else if &s[0..=1] == "DH" {
      let efficacy_s = &s[2..=8].parse::<usize>().map_err(|_| ())?;
      let efficacy = if *efficacy_s == 0 {
        LawEfficacy::CabinetOrder
      } else if *efficacy_s == 1000000 {
        LawEfficacy::Law
      } else {
        return Err(());
      };
      let num = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(DajokanHutatsu { efficacy, num })
    } else if &s[0..=0] == "M" {
      let ministry = match &s[1..=1] {
        "1" => {
          let n = usize::from_str_radix(&s[2..=8], 16).map_err(|_| ())?;
          let byte_s = format!("{n:028b}");
          let l = list_from_str::<M1Ministry>(&byte_s)?;
          Ministry::M1(l)
        }
        "2" => {
          let n = usize::from_str_radix(&s[2..=8], 16).map_err(|_| ())?;
          let byte_s = format!("{n:028b}");
          let l = list_from_str::<M2Ministry>(&byte_s)?;
          Ministry::M2(l)
        }
        "3" => {
          let n = usize::from_str_radix(&s[2..=8], 16).map_err(|_| ())?;
          let byte_s = format!("{n:028b}");
          let l = list_from_str::<M3Ministry>(&byte_s)?;
          Ministry::M3(l)
        }
        "4" => {
          let n = usize::from_str_radix(&s[2..=8], 16).map_err(|_| ())?;
          let byte_s = format!("{n:028b}");
          let l = list_from_str::<M4Ministry>(&byte_s)?;
          Ministry::M4(l)
        }
        "5" => {
          let n = usize::from_str_radix(&s[2..=8], 16).map_err(|_| ())?;
          let byte_s = format!("{n:028b}");
          let l = list_from_str::<M5Ministry>(&byte_s)?;
          Ministry::M5(l)
        }
        "6" => {
          let n = usize::from_str_radix(&s[2..=8], 16).map_err(|_| ())?;
          let byte_s = format!("{n:028b}");
          let l = list_from_str::<M6Ministry>(&byte_s)?;
          Ministry::M6(l)
        }
        _ => return Err(()),
      };
      let num = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(MinistryOrder { ministry, num })
    } else if &s[0..=3] == "RJNJ" {
      let kind = s[4..=5].parse::<usize>().map_err(|_| ())?;
      let kind_serial_number = s[6..=8].parse::<usize>().map_err(|_| ())?;
      let amendment_serial_number = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(Jinjin {
        kind,
        kind_serial_number,
        amendment_serial_number,
      })
    } else if &s[0..=3] == "RPMD" {
      let month = s[4..=5].parse::<usize>().map_err(|_| ())?;
      let day = s[6..=7].parse::<usize>().map_err(|_| ())?;
      let num = s[8..=11].parse::<usize>().map_err(|_| ())?;
      Ok(PrimeMinisterDecision { month, day, num })
    } else if &s[0..=0] == "R" {
      let institution_s = &s[1..=8].parse::<usize>().map_err(|_| ())?;
      let institution = Institution::from_int(*institution_s).ok_or(())?;
      let num = s[9..=11].parse::<usize>().map_err(|_| ())?;
      Ok(Regulation { institution, num })
    } else {
      Err(())
    }
  }
}

/// 法令ID： <https://elaws.e-gov.go.jp/file/LawIdNamingConvention.pdf>を参照
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct LawId {
  era: Era,
  year: usize,
  law_id_type: LawIdType,
}

impl Display for LawId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.era {
      Era::Meiji => write!(f, "1{:02}{}", self.year, self.law_id_type),
      Era::Taisho => write!(f, "2{:02}{}", self.year, self.law_id_type),
      Era::Showa => write!(f, "3{:02}{}", self.year, self.law_id_type),
      Era::Heisei => write!(f, "4{:02}{}", self.year, self.law_id_type),
      Era::Reiwa => write!(f, "5{:02}{}", self.year, self.law_id_type),
    }
  }
}

impl FromStr for LawId {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let era = match &s[0..=0] {
      "1" => Era::Meiji,
      "2" => Era::Taisho,
      "3" => Era::Showa,
      "4" => Era::Heisei,
      "5" => Era::Reiwa,
      _ => return Err(()),
    };
    let year = &s[1..=2].parse::<usize>().map_err(|_| ())?;
    let law_id_type = &s[3..=14].parse::<LawIdType>().map_err(|_| ())?;
    Ok(LawId {
      era,
      year: *year,
      law_id_type: law_id_type.clone(),
    })
  }
}

#[test]
fn check_from_str_law_id() {
  let s = "325M50001000004";
  let law_id = LawId::from_str(s).unwrap();
  assert_eq!(
    law_id,
    LawId {
      era: Era::Showa,
      year: 25,
      law_id_type: LawIdType::MinistryOrder {
        ministry: Ministry::M5(vec![
          M5Ministry::MinistryOfPostsAndTelecommunicationsOrdinance
        ]),
        num: 4
      }
    }
  );
  assert_eq!(law_id.to_string(), s);
}

#[test]
fn check_from_str_law_id_2() {
  let s = "345AC0000000089";
  let law_id = LawId::from_str(s).unwrap();
  assert_eq!(
    law_id,
    LawId {
      era: Era::Showa,
      year: 45,
      law_id_type: LawIdType::Act {
        rippou_type: RippouType::Kakuhou,
        num: 89
      }
    }
  );
  assert_eq!(law_id.to_string(), s);
}

/// 法令のデータ
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct LawData {
  /// 制定年月日
  pub date: Date,
  /// 法令名
  pub name: String,
  /// 法令番号
  pub num: String,
  /// 法令ID
  /// <https://elaws.e-gov.go.jp/file/LawIdNamingConvention.pdf>を参照
  pub id: LawId,
  /// 改正法令の情報
  pub patch: Vec<LawPatchInfo>,
}

/// 改正法令の情報
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct LawPatchInfo {
  /// 法令ID
  pub id: LawId,
  /// 改正・成立年月日
  pub patch_date: Date,
  /// 改正した法令の名前（成立法の場合はNone）
  #[serde(skip_serializing_if = "Option::is_none")]
  pub patch_id: Option<LawId>,
}

impl FromStr for LawPatchInfo {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let sl = s.split('_').collect::<Vec<&str>>();
    let id_s = sl.first().ok_or(())?;
    let id = LawId::from_str(id_s)?;
    let date_s = sl.get(1).ok_or(())?;
    let y = date_s[0..=3].parse::<usize>().map_err(|_| ())?;
    let m = date_s[4..=5].parse::<usize>().map_err(|_| ())?;
    let d = date_s[6..=7].parse::<usize>().map_err(|_| ())?;
    let patch_date = Date::gen_from_ad(y, m, d);
    let patch_s = sl.get(2).ok_or(())?;
    let patch_id = if patch_s == &"000000000000000" {
      None
    } else {
      Some(LawId::from_str(patch_s)?)
    };
    Ok(LawPatchInfo {
      id,
      patch_date,
      patch_id,
    })
  }
}
