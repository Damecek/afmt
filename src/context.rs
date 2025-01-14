use crate::config::Config;
use crate::utility::visit_root;
use anyhow::Result;
use colored::Colorize;
use tree_sitter::{Language, Node, Parser, Tree};

#[derive(Clone)]
pub struct FmtContext<'a> {
    pub config: &'a Config,
    pub source_code: String,
    pub ast_tree: Tree,
}

impl<'a> FmtContext<'a> {
    pub fn new(config: &'a Config, source_code: String) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading Apex grammar");

        let ast_tree = parser.parse(&source_code, None).unwrap();
        let root_node = &ast_tree.root_node();

        if root_node.has_error() {
            if let Some(error_node) = Self::find_last_error_node(root_node) {
                let error_snippet = &source_code[error_node.start_byte()..error_node.end_byte()];
                println!(
                    "Error in node kind: {}, at byte range: {}-{}, snippet: {}",
                    error_node.kind().yellow(),
                    error_node.start_byte(),
                    error_node.end_byte(),
                    error_snippet
                );
            }
            panic!("{}", "Parser encounters an error node in the tree.".red());
        }

        Self {
            config,
            source_code,
            ast_tree,
        }
    }

    pub fn format_one_file(&self) -> Result<String> {
        let mut result = String::new();
        result.push_str(&visit_root(self));

        // add file ending new line;
        result.push('\n');

        Ok(result)
    }

    fn find_last_error_node<'tree>(node: &Node<'tree>) -> Option<Node<'tree>> {
        if !node.has_error() {
            return None; // If the current node has no error, return None
        }

        let mut last_error_node = Some(*node);

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.has_error() {
                    last_error_node = Self::find_last_error_node(&child);
                }
            }
        }

        last_error_node // Return the last (deepest) error node
    }
}

extern "C" {
    fn tree_sitter_apex() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_apex() }
}
