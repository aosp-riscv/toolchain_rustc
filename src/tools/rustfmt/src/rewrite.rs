// A generic trait to abstract the rewriting of an element (of the AST).

use std::cell::RefCell;
use std::rc::Rc;

use syntax::parse::ParseSess;
use syntax::ptr;
use syntax::source_map::{SourceMap, Span};

use crate::config::{Config, IndentStyle};
use crate::shape::Shape;
use crate::skip::SkipContext;
use crate::visitor::SnippetProvider;
use crate::FormatReport;

pub(crate) trait Rewrite {
    /// Rewrite self into shape.
    fn rewrite(&self, context: &RewriteContext<'_>, shape: Shape) -> Option<String>;
}

impl<T: Rewrite> Rewrite for ptr::P<T> {
    fn rewrite(&self, context: &RewriteContext<'_>, shape: Shape) -> Option<String> {
        (**self).rewrite(context, shape)
    }
}

#[derive(Clone)]
pub(crate) struct RewriteContext<'a> {
    pub(crate) parse_session: &'a ParseSess,
    pub(crate) source_map: &'a SourceMap,
    pub(crate) config: &'a Config,
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    pub(crate) inside_macro: RefCell<bool>,
=======
    pub(crate) inside_macro: Rc<RefCell<bool>>,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    // Force block indent style even if we are using visual indent style.
    pub(crate) use_block: RefCell<bool>,
    // When `is_if_else_block` is true, unindent the comment on top
    // of the `else` or `else if`.
    pub(crate) is_if_else_block: RefCell<bool>,
    // When rewriting chain, veto going multi line except the last element
    pub(crate) force_one_line_chain: RefCell<bool>,
    pub(crate) snippet_provider: &'a SnippetProvider<'a>,
    // Used for `format_snippet`
    pub(crate) macro_rewrite_failure: RefCell<bool>,
    pub(crate) report: FormatReport,
    pub(crate) skip_context: SkipContext,
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
    pub(crate) skipped_range: Rc<RefCell<Vec<(usize, usize)>>>,
}

pub(crate) struct InsideMacroGuard {
    is_nested_macro_context: bool,
    inside_macro_ref: Rc<RefCell<bool>>,
}

impl InsideMacroGuard {
    pub(crate) fn is_nested(&self) -> bool {
        self.is_nested_macro_context
    }
}

impl Drop for InsideMacroGuard {
    fn drop(&mut self) {
        self.inside_macro_ref.replace(self.is_nested_macro_context);
    }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

impl<'a> RewriteContext<'a> {
    pub(crate) fn snippet(&self, span: Span) -> &str {
        self.snippet_provider.span_to_snippet(span).unwrap()
    }

    /// Returns `true` if we should use block indent style for rewriting function call.
    pub(crate) fn use_block_indent(&self) -> bool {
        self.config.indent_style() == IndentStyle::Block || *self.use_block.borrow()
    }

    pub(crate) fn budget(&self, used_width: usize) -> usize {
        self.config.max_width().saturating_sub(used_width)
    }

    pub(crate) fn inside_macro(&self) -> bool {
        *self.inside_macro.borrow()
    }

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
    pub(crate) fn enter_macro(&self) -> InsideMacroGuard {
        let is_nested_macro_context = self.inside_macro.replace(true);
        InsideMacroGuard {
            is_nested_macro_context,
            inside_macro_ref: self.inside_macro.clone(),
        }
    }

    pub(crate) fn leave_macro(&self) {
        self.inside_macro.replace(false);
    }

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    pub(crate) fn is_if_else_block(&self) -> bool {
        *self.is_if_else_block.borrow()
    }
}
