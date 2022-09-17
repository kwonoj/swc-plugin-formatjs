use serde::Serialize;

/// Subset of options that will be parsed from the ICU message number skeleton.
#[derive(Default, Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsIntlNumberFormatOptions {
    // TODO
}

/// Subset of options that will be parsed from the ICU message daet or time skeleton.
#[derive(Default, Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsIntlDateTimeFormatOptions {
    // TODO
}
