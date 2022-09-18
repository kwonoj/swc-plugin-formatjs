use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CompactDisplay {
    Short,
    Long,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Notation {
    Standard,
    Scientific,
    Engineering,
    Compact,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UnitDisplay {
    Short,
    Long,
    Narrow,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormatOptionsTrailingZeroDisplay {
    Auto,
    StripIfInteger,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormatOptionsRoundingPriority {
    Auto,
    MorePrecision,
    LessPrecision,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LocaleMatcherFormatOptions {
    Lookup,
    #[serde(rename = "best fit")]
    BestFit,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormatOptionsStyle {
    Decimal,
    Percent,
    Currency,
    Unit,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormatOptionsCurrencyDisplay {
    Symbol,
    Code,
    Name,
    NarrowSymbol,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormatOptionsCurrencySign {
    Standard,
    Accounting,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormatOptionsSignDisplay {
    Auto,
    Always,
    Never,
    ExceptZero,
}

/// Subset of options that will be parsed from the ICU message number skeleton.
#[derive(Default, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsIntlNumberFormatOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notation: Option<Notation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compact_display: Option<CompactDisplay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_matcher: Option<LocaleMatcherFormatOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<NumberFormatOptionsStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_sign: Option<NumberFormatOptionsCurrencySign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_display: Option<NumberFormatOptionsSignDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numbering_system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_zero_display: Option<NumberFormatOptionsTrailingZeroDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounding_priority: Option<NumberFormatOptionsRoundingPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_grouping: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_integer_digits: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_fraction_digits: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_fraction_digits: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_significant_digits: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_significant_digits: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_display: Option<NumberFormatOptionsCurrencyDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_display: Option<UnitDisplay>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DateTimeFormatMatcher {
    Basic,
    #[serde(rename = "best fit")]
    BestFit,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DateTimeFormatStyle {
    Full,
    Long,
    Medium,
    Short,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DateTimeDisplayFormat {
    Numeric,
    #[serde(rename = "2-digit")]
    TwoDigit,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DateTimeMonthDisplayFormat {
    Numeric,
    #[serde(rename = "2-digit")]
    TwoDigit,
    Long,
    Short,
    Narrow
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeZoneNameFormat {
    Short,
    Long,
    ShortOffset,
    LongOffset,
    ShortGeneric,
    LongGeneric,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]  
pub enum HourCycle {
    H11,
    H12,
    H23,
    H24
}

/// Subset of options that will be parsed from the ICU message daet or time skeleton.
#[derive(Default, Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsIntlDateTimeFormatOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_matcher: Option<LocaleMatcherFormatOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekday: Option<UnitDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub era: Option<UnitDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<DateTimeDisplayFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<DateTimeMonthDisplayFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<DateTimeDisplayFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<DateTimeDisplayFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<DateTimeDisplayFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<DateTimeDisplayFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_name: Option<TimeZoneNameFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour12: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour_cycle: Option<HourCycle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_matcher: Option<DateTimeFormatMatcher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_style: Option<DateTimeFormatStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_style: Option<DateTimeFormatStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_period: Option<UnitDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fractional_second_digits: Option<usize>,
}
