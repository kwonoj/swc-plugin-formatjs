use swc_core::{
    ecma::{ast::Program, visit::*},
    plugin::{
        plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};
use swc_formatjs_visitor::create_formatjs_visitor;

#[plugin_transform]
pub fn process(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    let visitor = create_formatjs_visitor();

    program.fold_with(&mut as_folder(visitor))
}
