use super::js_intl::*;
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Serialize, Serializer};
use std::fmt;

/// The type of an error that occurred while building an AST.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Argument is unclosed (e.g. `{0`)
    ExpectArgumentClosingBrace,
    /// Argument is empty (e.g. `{}`).
    EmptyArgument,
    /// Argument is malformed (e.g. `{foo!}``)
    MalformedArgument,
    /// Expect an argument type (e.g. `{foo,}`)
    ExpectArgumentType,
    /// Unsupported argument type (e.g. `{foo,foo}`)
    InvalidArgumentType,
    /// Expect an argument style (e.g. `{foo, number, }`)
    ExpectArgumentStyle,
    /// The number skeleton is invalid.
    InvalidNumberSkeleton,
    /// The date time skeleton is invalid.
    InvalidDateTimeSkeleton,
    /// Exepct a number skeleton following the `::` (e.g. `{foo, number, ::}`)
    ExpectNumberSkeleton,
    /// Exepct a date time skeleton following the `::` (e.g. `{foo, date, ::}`)
    ExpectDateTimeSkeleton,
    /// Unmatched apostrophes in the argument style (e.g. `{foo, number, 'test`)
    UnclosedQuoteInArgumentStyle,
    /// Missing select argument options (e.g. `{foo, select}`)
    ExpectSelectArgumentOptions,

    /// Expecting an offset value in `plural` or `selectordinal` argument (e.g `{foo, plural, offset}`)
    ExpectPluralArgumentOffsetValue,
    /// Offset value in `plural` or `selectordinal` is invalid (e.g. `{foo, plural, offset: x}`)
    InvalidPluralArgumentOffsetValue,

    /// Expecting a selector in `select` argument (e.g `{foo, select}`)
    ExpectSelectArgumentSelector,
    /// Expecting a selector in `plural` or `selectordinal` argument (e.g `{foo, plural}`)
    ExpectPluralArgumentSelector,

    /// Expecting a message fragment after the `select` selector (e.g. `{foo, select, apple}`)
    ExpectSelectArgumentSelectorFragment,
    /// Expecting a message fragment after the `plural` or `selectordinal` selector
    /// (e.g. `{foo, plural, one}`)
    ExpectPluralArgumentSelectorFragment,

    /// Selector in `plural` or `selectordinal` is malformed (e.g. `{foo, plural, =x {#}}`)
    InvalidPluralArgumentSelector,

    /// Duplicate selectors in `plural` or `selectordinal` argument.
    /// (e.g. {foo, plural, one {#} one {#}})
    DuplicatePluralArgumentSelector,
    /// Duplicate selectors in `select` argument.
    /// (e.g. {foo, select, apple {apple} apple {apple}})
    DuplicateSelectArgumentSelector,

    /// Plural or select argument option must have `other` clause.
    MissingOtherClause,

    /// The tag is malformed. (e.g. `<bold!>foo</bold!>)
    InvalidTag,
    /// The closing tag does not match the opening tag. (e.g. `<bold>foo</italic>`)
    UnmatchedClosingTag,
    /// The opening tag has unmatched closing tag. (e.g. `<bold>foo`)
    UnclosedTag,
}

/// A single position in an ICU message.
///
/// A position encodes one half of a span, and include the code unit offset, line
/// number and column number.
#[derive(Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(offset: usize, line: usize, column: usize) -> Position {
        Position { offset, line, column }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position::new({:?}, {:?}, {:?})", self.offset, self.line, self.column)
    }
}

/// Span represents the position information of a single AST item.
///
/// All span positions are absolute byte offsets that can be used on the
/// original regular expression that was parsed.
#[derive(Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Span::new({:?}, {:?})", self.start, self.end)
    }
}

impl Span {
    /// Create a new span with the given positions.
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
}

/// An error that occurred while parsing an ICU message into an abstract
/// syntax tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    pub kind: ErrorKind,
    /// The original message that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pub message: String,
    /// The span of this error.
    pub span: Span,
}

/// An abstract syntax tree for a ICU message. Adapted from:
/// https://github.com/formatjs/formatjs/blob/c03d4989323a33765798acdd74fb4f5b01f0bdcd/packages/intl-messageformat-parser/src/types.ts
pub type Ast<'s> = Vec<AstElement<'s>>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluralType {
    Cardinal,
    Ordinal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AstElement<'s> {
    /// Raw text
    Literal { value: String, span: Span },
    /// Variable w/o any format, e.g `var` in `this is a {var}`
    Argument { value: &'s str, span: Span },
    /// Variable w/ number format
    Number { value: &'s str, span: Span, style: Option<NumberArgStyle<'s>> },
    /// Variable w/ date format
    Date { value: &'s str, span: Span, style: Option<DateTimeArgStyle<'s>> },
    /// Variable w/ time format
    Time { value: &'s str, span: Span, style: Option<DateTimeArgStyle<'s>> },
    /// Variable w/ select format
    Select { value: &'s str, span: Span, options: PluralOrSelectOptions<'s> },
    /// Variable w/ plural format
    Plural {
        value: &'s str,
        plural_type: PluralType,
        span: Span,
        // TODO: want to use double here but it does not implement Eq trait.
        offset: i64,
        options: PluralOrSelectOptions<'s>,
    },
    /// Only possible within plural argument.
    /// This is the `#` symbol that will be substituted with the count.
    Pound(Span),
    /// XML-like tag
    Tag { value: &'s str, span: Span, children: Box<Ast<'s>> },
}

// Until this is resolved, we have to roll our own serialization: https://github.com/serde-rs/serde/issues/745
impl<'s> Serialize for AstElement<'s> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            AstElement::Literal { ref value, ref span } => {
                let mut state = serializer.serialize_struct("Literal", 3)?;
                state.serialize_field("type", &0)?;
                state.serialize_field("value", value)?;
                state.serialize_field("location", span)?;
                state.end()
            }
            AstElement::Argument { ref value, ref span } => {
                let mut state = serializer.serialize_struct("Argument", 3)?;
                state.serialize_field("type", &1)?;
                state.serialize_field("value", value)?;
                state.serialize_field("location", span)?;
                state.end()
            }
            AstElement::Number { ref value, ref span, ref style } => {
                let mut state = serializer.serialize_struct("Number", 4)?;
                state.serialize_field("type", &2)?;
                state.serialize_field("value", value)?;
                state.serialize_field("location", span)?;
                state.serialize_field("style", style)?;
                state.end()
            }
            AstElement::Date { ref value, ref span, ref style } => {
                let mut state = serializer.serialize_struct("Date", 4)?;
                state.serialize_field("type", &3)?;
                state.serialize_field("value", value)?;
                state.serialize_field("location", span)?;
                state.serialize_field("style", style)?;
                state.end()
            }
            AstElement::Time { ref value, ref span, ref style } => {
                let mut state = serializer.serialize_struct("Time", 4)?;
                state.serialize_field("type", &4)?;
                state.serialize_field("value", value)?;
                state.serialize_field("location", span)?;
                state.serialize_field("style", style)?;
                state.end()
            }
            AstElement::Select { ref value, ref span, ref options } => {
                let mut state = serializer.serialize_struct("Select", 4)?;
                state.serialize_field("type", &5)?;
                state.serialize_field("value", value)?;
                state.serialize_field("location", span)?;
                state.serialize_field("style", options)?;
                state.end()
            }
            AstElement::Plural {
                ref value,
                ref span,
                ref plural_type,
                ref offset,
                ref options,
            } => {
                let mut state = serializer.serialize_struct("Plural", 6)?;
                state.serialize_field("type", &6)?;
                state.serialize_field("value", value)?;
                state.serialize_field("type", plural_type)?;
                state.serialize_field("location", span)?;
                state.serialize_field("offset", offset)?;
                state.serialize_field("style", options)?;
                state.end()
            }
            AstElement::Pound(ref span) => {
                let mut state = serializer.serialize_struct("Pound", 2)?;
                state.serialize_field("type", &7)?;
                state.serialize_field("location", span)?;
                state.end()
            }
            AstElement::Tag { ref value, ref span, ref children } => {
                let mut state = serializer.serialize_struct("Pound", 2)?;
                state.serialize_field("type", &8)?;
                state.serialize_field("location", span)?;
                state.serialize_field("value", value)?;
                state.serialize_field("children", children)?;
                state.end()
            }
        }
    }
}

/// Workaround of Rust's orphan impl rule
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluralOrSelectOptions<'s>(pub Vec<(&'s str, PluralOrSelectOption<'s>)>);

impl<'s> Serialize for PluralOrSelectOptions<'s> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let options = &self.0;
        let mut state = serializer.serialize_map(Some(options.len()))?;
        for (selector, fragment) in options {
            state.serialize_entry(selector, fragment)?;
        }
        state.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NumberArgStyle<'s> {
    Style(&'s str),
    Skeleton(NumberSkeleton<'s>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberSkeleton<'s> {
    pub tokens: Vec<NumberSkeletonToken<'s>>,
    pub span: Span,
    pub parsed_options: Option<JsIntlNumberFormatOptions>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberSkeletonToken<'s> {
    pub stem: &'s str,
    pub options: Vec<&'s str>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DateTimeArgStyle<'s> {
    Style(&'s str),
    Skeleton(DateTimeSkeleton<'s>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeSkeleton<'s> {
    pub pattern: &'s str,
    pub span: Span,
    pub parsed_options: Option<JsIntlDateTimeFormatOptions>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluralOrSelectOption<'s> {
    pub value: Ast<'s>,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_intl::JsIntlNumberFormatOptions;
    use serde_json::json;

    #[test]
    fn serialize_number_arg_style_with_skeleton() {
        assert_eq!(
            serde_json::to_value(NumberArgStyle::Skeleton(NumberSkeleton {
                tokens: vec![NumberSkeletonToken { stem: "foo", options: vec!["bar", "baz"] }],
                span: Span::new(Position::new(0, 1, 1), Position::new(11, 1, 12)),
                parsed_options: Some(JsIntlNumberFormatOptions {}),
            }))
            .unwrap(),
            json!({
                "tokens": [{
                    "stem": "foo",
                    "options": [
                        "bar",
                        "baz"
                    ]
                }],
                "span": {
                    "start": {
                        "offset": 0,
                        "line": 1,
                        "column": 1,
                    },
                    "end": {
                        "offset": 11,
                        "line": 1,
                        "column": 12,
                    }
                },
                "parsedOptions": {},
            })
        );
    }

    #[test]
    fn serialize_number_arg_style_string() {
        assert_eq!(
            serde_json::to_value(NumberArgStyle::Style("percent")).unwrap(),
            json!("percent")
        )
    }

    #[test]
    fn serialize_plural_type() {
        assert_eq!(serde_json::to_value(PluralType::Cardinal).unwrap(), json!("cardinal"))
    }
}
