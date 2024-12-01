use crate::lang::Language;
use anyhow::anyhow;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::Date;

pub mod post_provider;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct MyDate(Date);

pub(crate) fn parse_file_name(file_name: &str) -> anyhow::Result<(u32, Language, &str)> {
  let mut split = file_name.split('.');

  let idx = split
    .next()
    .ok_or_else(|| anyhow!("Index missing in file name {}", file_name))?
    .parse()?;
  let slug = split
    .next()
    .ok_or_else(|| anyhow!("Slug missing in file name {}", file_name))?;
  let lang = split
    .next()
    .ok_or_else(|| anyhow!("Language missing in file name {}", file_name))?
    .try_into()?;

  Ok((idx, lang, slug))
}

impl Serialize for MyDate {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = format!(
      "{:0>4}-{:0>2}-{:0>2}",
      self.0.year(),
      self.0.month() as u8,
      self.0.day()
    );

    serializer.serialize_str(&s)
  }
}

impl<'de> Deserialize<'de> for MyDate {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let mut split = s.split('-');

    let year = split
      .next()
      .ok_or_else(|| Error::custom(format!("Invalid date format {}", s)))?
      .parse()
      .map_err(|e| Error::custom(format!("{}", e)))?;

    let month: u8 = split
      .next()
      .ok_or_else(|| Error::custom(format!("Invalid date format {}", s)))?
      .parse()
      .map_err(|e| Error::custom(format!("{}", e)))?;

    let day = split
      .next()
      .ok_or_else(|| Error::custom(format!("Invalid date format {}", s)))?
      .parse()
      .map_err(|e| Error::custom(format!("{}", e)))?;

    Date::from_calendar_date(
      year,
      month
        .try_into()
        .map_err(|e| Error::custom(format!("{}", e)))?,
      day,
    )
    .map_err(|e| Error::custom(format!("{}", e)))
    .map(MyDate)
  }
}
