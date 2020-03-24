use serde::Serialize;
use swc_common;
use swc_common::Span;
use swc_ecma_ast;

use super::parser::DocParser;
use super::DocNode;
use super::DocNodeKind;

#[derive(Debug, Serialize)]
pub struct NamespaceDef {
  pub elements: Vec<DocNode>,
}

pub fn get_doc_for_ts_namespace_decl(
  doc_parser: &DocParser,
  ts_namespace_decl: &swc_ecma_ast::TsNamespaceDecl,
) -> DocNode {
  let js_doc = doc_parser.js_doc_for_span(ts_namespace_decl.span);
  let snippet = doc_parser
    .source_map
    .span_to_snippet(ts_namespace_decl.span)
    .expect("Snippet not found")
    .trim_end()
    .to_string();

  let namespace_name = ts_namespace_decl.id.sym.to_string();

  use swc_ecma_ast::TsNamespaceBody::*;

  let elements = match &*ts_namespace_decl.body {
    TsModuleBlock(ts_module_block) => {
      super::module::get_doc_nodes_for_module_body(doc_parser, ts_module_block.body.clone())
    }
    TsNamespaceDecl(ts_namespace_decl) => {
      vec![get_doc_for_ts_namespace_decl(doc_parser, ts_namespace_decl)]
    }
  };

  let ns_def = NamespaceDef { elements };

  DocNode {
    kind: DocNodeKind::Namespace,
    name: namespace_name,
    snippet,
    location: doc_parser
      .source_map
      .lookup_char_pos(ts_namespace_decl.span.lo())
      .into(),
    js_doc,
    function_def: None,
    variable_def: None,
    enum_def: None,
    class_def: None,
    type_alias_def: None,
    namespace_def: Some(ns_def),
    interface_def: None,
  }
}

pub fn get_doc_for_ts_module(
  doc_parser: &DocParser,
  parent_span: Span,
  ts_module_decl: &swc_ecma_ast::TsModuleDecl,
) -> DocNode {
  let js_doc = doc_parser.js_doc_for_span(parent_span);
  let snippet = doc_parser
    .source_map
    .span_to_snippet(parent_span)
    .expect("Snippet not found")
    .trim_end()
    .to_string();

  use swc_ecma_ast::TsModuleName;
  let namespace_name = match &ts_module_decl.id {
    TsModuleName::Ident(ident) => ident.sym.to_string(),
    TsModuleName::Str(str_) => str_.value.to_string(),
  };

  let elements = if let Some(body) = &ts_module_decl.body {
    use swc_ecma_ast::TsNamespaceBody::*;

    match &body {
      TsModuleBlock(ts_module_block) => {
        super::module::get_doc_nodes_for_module_body(doc_parser, ts_module_block.body.clone())
      }
      TsNamespaceDecl(ts_namespace_decl) => {
        vec![get_doc_for_ts_namespace_decl(doc_parser, ts_namespace_decl)]
      }
    }
  } else {
    vec![]
  };

  let ns_def = NamespaceDef { elements };

  DocNode {
    kind: DocNodeKind::Namespace,
    name: namespace_name,
    snippet,
    location: doc_parser
      .source_map
      .lookup_char_pos(parent_span.lo())
      .into(),
    js_doc,
    function_def: None,
    variable_def: None,
    enum_def: None,
    class_def: None,
    type_alias_def: None,
    namespace_def: Some(ns_def),
    interface_def: None,
  }
}
