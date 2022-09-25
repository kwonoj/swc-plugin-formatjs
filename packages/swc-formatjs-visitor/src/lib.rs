use std::collections::HashMap;

use icu_messageformat_parser::{Parser, ParserOptions};
use once_cell::sync::Lazy;
use regex::Regex as Regexp;
use serde::{ser::SerializeMap, Deserialize, Serialize};
use swc_core::{
    common::{
        comments::{Comment, CommentKind, Comments},
        source_map::Pos,
        BytePos, Loc, SourceMapper, Span, Spanned, DUMMY_SP,
    },
    ecma::{
        ast::{
            Expr, Ident, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXExpr,
            JSXNamespacedName, JSXOpeningElement, Lit, ModuleItem, ObjectLit, Prop, PropName,
            PropOrSpread, Str,
        },
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

pub static WHITESPACE_REGEX: Lazy<Regexp> = Lazy::new(|| Regexp::new(r"\s+").unwrap());

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FormatJSPluginOptions {
    pub pragma: String,
    pub remove_default_message: bool,
    pub id_interpolate_pattern: Option<String>,
    pub ast: bool,
    pub extract_source_location: bool,
    pub preserve_whitespace: bool,
    pub __debug_extracted_messages_comment: bool,
}

type Unknown = String;

#[derive(Debug, Clone, Default)]
pub struct JSXMessageDescriptorPath {
    id: Option<JSXAttrValue>,
    default_message: Option<JSXAttrValue>,
    description: Option<JSXAttrValue>,
}

#[derive(Debug, Clone, Default)]
pub struct MessageDescriptor {
    id: Option<String>,
    default_message: Option<String>,
    description: Option<MessageDescriptionValue>,
}

fn get_message_descriptor_key_from_jsx(name: &JSXAttrName) -> &str {
    match name {
        JSXAttrName::Ident(name)
        | JSXAttrName::JSXNamespacedName(JSXNamespacedName { name, .. }) => &*name.sym,
    }

    // Do not support evaluatePath()
}

fn create_message_descriptor_from_jsx_attr(
    attrs: &Vec<JSXAttrOrSpread>,
) -> JSXMessageDescriptorPath {
    let mut ret = JSXMessageDescriptorPath::default();
    for attr in attrs {
        if let JSXAttrOrSpread::JSXAttr(JSXAttr { name, value, .. }) = attr {
            let key = get_message_descriptor_key_from_jsx(name);

            match key {
                "id" => {
                    ret.id = value.clone();
                }
                "defaultMessage" => {
                    ret.default_message = value.clone();
                }
                "description" => {
                    ret.description = value.clone();
                }
                _ => {
                    //unexpected
                }
            }
        }
    }

    ret
}

fn get_jsx_message_descriptor_value(
    value: &Option<JSXAttrValue>,
    is_message_node: Option<bool>,
) -> Option<String> {
    if value.is_none() {
        return None;
    }
    let value = value.as_ref().expect("Should be available");

    // NOTE: do not support evaluatePath
    match value {
        JSXAttrValue::JSXExprContainer(container) => {
            if is_message_node.unwrap_or(false) {
                if let JSXExpr::Expr(expr) = &container.expr {
                    // If this is already compiled, no need to recompiled it
                    if let Expr::Array(..) = &**expr {
                        return None;
                    }
                }
            }

            return match &container.expr {
                JSXExpr::Expr(expr) => match &**expr {
                    Expr::Lit(lit) => match &lit {
                        Lit::Str(str) => Some(str.value.to_string()),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            };
        }
        JSXAttrValue::Lit(lit) => match &lit {
            Lit::Str(str) => Some(str.value.to_string()),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum MessageDescriptionValue {
    Str(String),
    Obj(ObjectLit),
}

impl Serialize for MessageDescriptionValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageDescriptionValue::Str(str) => serializer.serialize_str(str),
            // NOTE: this is good enough to barely pass key-value object serialization. Not a complete implementation.
            MessageDescriptionValue::Obj(obj) => {
                let mut state = serializer.serialize_map(Some(obj.props.len()))?;
                for prop in &obj.props {
                    match prop {
                        PropOrSpread::Prop(prop) => {
                            match &**prop {
                                Prop::KeyValue(key_value) => {
                                    let key = match &key_value.key {
                                        PropName::Ident(ident) => ident.sym.to_string(),
                                        PropName::Str(str) => str.value.to_string(),
                                        _ => {
                                            //unexpected
                                            continue;
                                        }
                                    };
                                    let value = match &*key_value.value {
                                        Expr::Lit(lit) => match &lit {
                                            Lit::Str(str) => str.value.to_string(),
                                            _ => {
                                                //unexpected
                                                continue;
                                            }
                                        },
                                        _ => {
                                            //unexpected
                                            continue;
                                        }
                                    };
                                    state.serialize_entry(&key, &value)?;
                                }
                                _ => {
                                    //unexpected
                                    continue;
                                }
                            }
                        }
                        _ => {
                            //unexpected
                            continue;
                        }
                    }
                }
                state.end()
            }
        }
    }
}

// NOTE: due to not able to support static evaluation, this
// fn manually expands possible values for the description values
// from string to object.
fn get_jsx_message_descriptor_value_maybe_object(
    value: &Option<JSXAttrValue>,
    is_message_node: Option<bool>,
) -> Option<MessageDescriptionValue> {
    if value.is_none() {
        return None;
    }
    let value = value.as_ref().expect("Should be available");

    // NOTE: do not support evaluatePath
    match value {
        JSXAttrValue::JSXExprContainer(container) => {
            if is_message_node.unwrap_or(false) {
                if let JSXExpr::Expr(expr) = &container.expr {
                    // If this is already compiled, no need to recompiled it
                    if let Expr::Array(..) = &**expr {
                        return None;
                    }
                }
            }

            return match &container.expr {
                JSXExpr::Expr(expr) => match &**expr {
                    Expr::Lit(lit) => match &lit {
                        Lit::Str(str) => Some(MessageDescriptionValue::Str(str.value.to_string())),
                        _ => None,
                    },
                    Expr::Object(object_lit) => {
                        Some(MessageDescriptionValue::Obj(object_lit.clone()))
                    }
                    _ => None,
                },
                _ => None,
            };
        }
        JSXAttrValue::Lit(lit) => match &lit {
            Lit::Str(str) => Some(MessageDescriptionValue::Str(str.value.to_string())),
            _ => None,
        },
        _ => None,
    }
}

fn get_jsx_icu_message_value(
    message_path: &Option<JSXAttrValue>,
    preserve_whitespace: bool,
) -> String {
    if message_path.is_none() {
        return "".to_string();
    }

    let message =
        get_jsx_message_descriptor_value(message_path, Some(true)).unwrap_or("".to_string());

    let message = if preserve_whitespace {
        let message = WHITESPACE_REGEX.replace_all(&message, " ");
        message.trim().to_string()
    } else {
        message
    };

    let mut parser = Parser::new(message.as_str(), &ParserOptions::default());

    if let Err(e) = parser.parse() {
        let is_literal_err = if let Some(message_path) = message_path {
            if let JSXAttrValue::Lit(..) = message_path {
                if message.contains("\\\\") {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        // TODO: should use error emitter
        if is_literal_err {
            panic!(
                r#"
                    [React Intl] Message failed to parse.
                    It looks like `\\`s were used for escaping,
                    this won't work with JSX string literals.
                    Wrap with `{{}}`.
                    See: http://facebook.github.io/react/docs/jsx-gotchas.html
                    "#
            );
        } else {
            panic!(
                r#"
                    [React Intl] Message failed to parse.
                    See: https://formatjs.io/docs/core-concepts/icu-syntax
                    \n {:#?}
                    "#,
                e
            );
        }
    }

    return message;
}

fn interpolate_name(resource_path: &str, name: &str, content: &str) -> Option<String> {
    let filename = resource_path;

    let ext = "bin";
    let basename = "file";
    let directory = "";
    let folder = "";
    let query = "";

    /*
      if (resource_path) {
      const parsed = path.parse(loaderContext.resourcePath)
      let resourcePath = loaderContext.resourcePath

      if (parsed.ext) {
        ext = parsed.ext.slice(1)
      }

      if (parsed.dir) {
        basename = parsed.name
        resourcePath = parsed.dir + path.sep
      }

      if (typeof context !== 'undefined') {
        directory = path
          .relative(context, resourcePath + '_')
          .replace(/\\/g, '/')
          .replace(/\.\.(\/)?/g, '_$1')
        directory = directory.slice(0, -1)
      } else {
        directory = resourcePath.replace(/\\/g, '/').replace(/\.\.(\/)?/g, '_$1')
      }

      if (directory.length === 1) {
        directory = ''
      } else if (directory.length > 1) {
        folder = path.basename(directory)
      }
    }
      */

    let url = filename;

    /*
        if (content) {
      // Match hash template
      url = url
        // `hash` and `contenthash` are same in `loader-utils` context
        // let's keep `hash` for backward compatibility
        .replace(
          /\[(?:([^:\]]+):)?(?:hash|contenthash)(?::([a-z]+\d*))?(?::(\d+))?\]/gi,
          (_, hashType, digestType, maxLength) =>
            getHashDigest(content, hashType, digestType, parseInt(maxLength, 10))
        )
    }

    url = url
      .replace(/\[ext\]/gi, () => ext)
      .replace(/\[name\]/gi, () => basename)
      .replace(/\[path\]/gi, () => directory)
      .replace(/\[folder\]/gi, () => folder)
      .replace(/\[query\]/gi, () => query)
        */

    //return url
    Some(url.to_string())
}

fn evaluate_jsx_message_descriptor(
    descriptor_path: &JSXMessageDescriptorPath,
    options: &FormatJSPluginOptions,
    filename: &str,
) -> MessageDescriptor {
    let id = get_jsx_message_descriptor_value(&descriptor_path.id, None);
    let default_message = get_jsx_icu_message_value(
        &descriptor_path.default_message,
        options.preserve_whitespace,
    );

    let description =
        get_jsx_message_descriptor_value_maybe_object(&descriptor_path.description, None);

    // Note: do not support override fn
    let id = if id.is_some() && options.id_interpolate_pattern.is_some() && default_message != "" {
        let content = if let Some(description) = &description {
            if let MessageDescriptionValue::Str(description) = description {
                format!("{}{}", default_message, description)
            } else {
                default_message.clone()
            }
        } else {
            default_message.clone()
        };
        interpolate_name(
            filename,
            &options.id_interpolate_pattern.as_ref().unwrap(),
            &content,
        )
    } else {
        id
    };

    MessageDescriptor {
        id,
        default_message: Some(default_message),
        description,
    }
}

fn store_message(
    messages: &mut Vec<ExtractedMessage>,
    descriptor: &MessageDescriptor,
    filename: &str,
    location: Option<(Loc, Loc)>,
) {
    if descriptor.id.is_none() || descriptor.default_message.is_none() {
        // TODO: should use error emitter
        panic!("[React Intl] Message Descriptors require an `id` or `defaultMessage`.");
    }

    let source_location = if let Some(location) = location {
        let (start, end) = location;

        // NOTE: this is not fully identical to babel's test snapshot output
        Some(SourceLocation {
            file: filename.to_string(),
            start: Location {
                line: start.line,
                col: start.col.to_usize(),
            },
            end: Location {
                line: end.line,
                col: end.col.to_usize(),
            },
        })
    } else {
        None
    };

    messages.push(ExtractedMessage {
        id: descriptor.id.as_ref().expect("Should be available").clone(),
        default_message: descriptor
            .default_message
            .as_ref()
            .expect("Should be available")
            .clone(),
        description: descriptor.description.clone(),
        loc: source_location,
    });
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ExtractedMessage {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MessageDescriptionValue>,
    pub default_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc: Option<SourceLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    pub file: String,
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

pub struct FormatJSVisitor<C: Clone + Comments, S: SourceMapper> {
    // We may not need Arc in the plugin context - this is only to preserve isomorphic interface
    // between plugin & custom transform pass.
    source_map: std::sync::Arc<S>,
    comments: C,
    options: FormatJSPluginOptions,
    filename: String,
    messages: Vec<ExtractedMessage>,
    meta: HashMap<String, String>,
}

impl<C: Clone + Comments, S: SourceMapper> FormatJSVisitor<C, S> {
    fn read_pragma(&mut self, span_lo: BytePos, span_hi: BytePos) {
        let mut comments = self.comments.get_leading(span_lo).unwrap_or_default();
        comments.append(&mut self.comments.get_leading(span_hi).unwrap_or_default());

        let pragma = self.options.pragma.as_str();

        for comment in comments {
            let comment_text = &*comment.text;
            if comment_text.contains(pragma) {
                let value = comment_text.split(pragma).nth(1);
                if let Some(value) = value {
                    let value = WHITESPACE_REGEX.split(value.trim());
                    for kv in value {
                        let mut kv = kv.split(":");
                        self.meta.insert(
                            kv.next().unwrap().to_string(),
                            kv.next().unwrap().to_string(),
                        );
                    }
                }
            }
        }
    }
}

impl<C: Clone + Comments, S: SourceMapper> VisitMut for FormatJSVisitor<C, S> {
    noop_visit_mut_type!();

    fn visit_mut_jsx_opening_element(&mut self, jsx_opening_elem: &mut JSXOpeningElement) {
        let name = &jsx_opening_elem.name;

        let descriptor_path = create_message_descriptor_from_jsx_attr(&jsx_opening_elem.attrs);

        // In order for a default message to be extracted when
        // declaring a JSX element, it must be done with standard
        // `key=value` attributes. But it's completely valid to
        // write `<FormattedMessage {...descriptor} />`, because it will be
        // skipped here and extracted elsewhere. The descriptor will
        // be extracted only (storeMessage) if a `defaultMessage` prop.
        if descriptor_path.default_message.is_none() {
            return;
        }

        // Evaluate the Message Descriptor values in a JSX
        // context, then store it.
        let descriptor =
            evaluate_jsx_message_descriptor(&descriptor_path, &self.options, &self.filename);

        let source_location = if self.options.extract_source_location {
            Some((
                self.source_map.lookup_char_pos(jsx_opening_elem.span.lo),
                self.source_map.lookup_char_pos(jsx_opening_elem.span.hi),
            ))
        } else {
            None
        };

        store_message(
            &mut self.messages,
            &descriptor,
            &self.filename,
            source_location,
        );

        let id_attr: Option<&JSXAttr> = None;
        let first_attr = jsx_opening_elem.attrs.first().is_some();

        let mut attrs = vec![];
        for attr in jsx_opening_elem.attrs.drain(..) {
            match attr {
                JSXAttrOrSpread::JSXAttr(attr) => {
                    let key = get_message_descriptor_key_from_jsx(&attr.name);
                    match key {
                        "description" => {
                            // remove description
                        }
                        "defaultMessage" => {
                            if self.options.remove_default_message {
                                // remove defaultMessage
                            } else {
                                /*
                                if (ast && descriptor.defaultMessage) {
                                    defaultMessageAttr
                                        .get('value')
                                        .replaceWith(t.jsxExpressionContainer(t.nullLiteral()))
                                    const valueAttr = defaultMessageAttr.get(
                                        'value'
                                    ) as NodePath<t.JSXExpressionContainer>
                                    valueAttr
                                        .get('expression')
                                        .replaceWithSourceString(
                                        JSON.stringify(parse(descriptor.defaultMessage))
                                        )
                                    }
                                 */
                                attrs.push(JSXAttrOrSpread::JSXAttr(attr))
                            }
                        }
                        _ => attrs.push(JSXAttrOrSpread::JSXAttr(attr)),
                    }
                }
                _ => attrs.push(attr),
            }
        }

        jsx_opening_elem.attrs = attrs.to_vec();

        // Do not support overrideIdFn, only support idInterpolatePattern
        if descriptor.id.is_some() && self.options.id_interpolate_pattern.is_some() {
            if let Some(id_attr) = id_attr {
                id_attr.to_owned().value = Some(JSXAttrValue::Lit(Lit::Str(Str::from(
                    descriptor.id.unwrap(),
                ))));
            } else if first_attr {
                jsx_opening_elem.attrs.insert(
                    0,
                    JSXAttrOrSpread::JSXAttr(JSXAttr {
                        span: DUMMY_SP,
                        name: JSXAttrName::Ident(Ident::new("id".into(), DUMMY_SP)),
                        value: Some(JSXAttrValue::Lit(Lit::Str(Str::from(
                            descriptor.id.unwrap(),
                        )))),
                    }),
                )
            }
        }

        // tag_as_extracted();
    }

    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        /*
        if self.is_instrumented_already() {
            return;
        }
        */

        for item in items {
            self.read_pragma(item.span().lo, item.span().hi);
            item.visit_mut_children_with(self);
        }

        if self.options.__debug_extracted_messages_comment {
            let messages_json_str =
                serde_json::to_string(&self.messages).expect("Should be serializable");
            let meta_json_str = serde_json::to_string(&self.meta).expect("Should be serializable");

            // Append extracted messages to the end of the file as stringified JSON comments.
            // SWC's plugin does not support to return aribitary data other than transformed codes,
            // There's no way to pass extracted messages after transform.
            // This is not a public interface; currently for debugging / testing purpose only.
            self.comments.add_trailing(
                Span::dummy_with_cmt().hi,
                Comment {
                    kind: CommentKind::Block,
                    span: Span::dummy_with_cmt(),
                    text: format!(
                        "__formatjs__messages_extracted__::{{\"messages\":{}, \"meta\":{}}}",
                        messages_json_str, meta_json_str
                    )
                    .into(),
                },
            );
        }
    }
}

pub fn create_formatjs_visitor<C: Clone + Comments, S: SourceMapper>(
    source_map: std::sync::Arc<S>,
    comments: C,
    plugin_options: FormatJSPluginOptions,
    filename: &str,
) -> FormatJSVisitor<C, S> {
    FormatJSVisitor {
        source_map,
        comments,
        options: plugin_options,
        filename: filename.to_string(),
        messages: Default::default(),
        meta: Default::default(),
    }
}
