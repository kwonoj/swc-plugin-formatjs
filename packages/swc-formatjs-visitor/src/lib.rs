use swc_core::ecma::visit::VisitMut;

pub struct FormatJSVisitor {}

impl VisitMut for FormatJSVisitor {}

pub fn create_formatjs_visitor() -> FormatJSVisitor {
    FormatJSVisitor {}
}
