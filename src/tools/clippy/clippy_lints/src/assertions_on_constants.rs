use rustc::hir::{Expr, ExprKind};
use rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use rustc::{declare_lint_pass, declare_tool_lint};
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use syntax_pos::Span;
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

use crate::consts::{constant, Constant};
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use crate::utils::{in_macro_or_desugar, is_direct_expn_of, span_help_and_lint};
=======
use crate::utils::{is_direct_expn_of, is_expn_of, span_help_and_lint};
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

declare_clippy_lint! {
    /// **What it does:** Checks for `assert!(true)` and `assert!(false)` calls.
    ///
    /// **Why is this bad?** Will be optimized out by the compiler or should probably be replaced by a
    /// panic!() or unreachable!()
    ///
    /// **Known problems:** None
    ///
    /// **Example:**
    /// ```rust,ignore
    /// assert!(false)
    /// // or
    /// assert!(true)
    /// // or
    /// const B: bool = false;
    /// assert!(B)
    /// ```
    pub ASSERTIONS_ON_CONSTANTS,
    style,
    "`assert!(true)` / `assert!(false)` will be optimized out by the compiler, and should probably be replaced by a `panic!()` or `unreachable!()`"
}

declare_lint_pass!(AssertionsOnConstants => [ASSERTIONS_ON_CONSTANTS]);

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for AssertionsOnConstants {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, e: &'tcx Expr) {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        let mut is_debug_assert = false;
        let debug_assert_not_in_macro_or_desugar = |span: Span| {
            is_debug_assert = true;
            // Check that `debug_assert!` itself is not inside a macro
            !in_macro_or_desugar(span)
        };
        if_chain! {
            if let Some(assert_span) = is_direct_expn_of(e.span, "assert");
            if !in_macro_or_desugar(assert_span)
                || is_direct_expn_of(assert_span, "debug_assert")
                    .map_or(false, debug_assert_not_in_macro_or_desugar);
            if let ExprKind::Unary(_, ref lit) = e.node;
            if let Some(bool_const) = constant(cx, cx.tables, lit);
            then {
                match bool_const.0 {
                    Constant::Bool(true) => {
                        span_help_and_lint(
                            cx,
                            ASSERTIONS_ON_CONSTANTS,
                            e.span,
                            "`assert!(true)` will be optimized out by the compiler",
                            "remove it"
                        );
                    },
                    Constant::Bool(false) if !is_debug_assert => {
                        span_help_and_lint(
                            cx,
                            ASSERTIONS_ON_CONSTANTS,
                            e.span,
                            "`assert!(false)` should probably be replaced",
                            "use `panic!()` or `unreachable!()`"
                        );
                    },
                    _ => (),
=======
        let lint_assert_cb = |is_debug_assert: bool| {
            if let ExprKind::Unary(_, ref lit) = e.node {
                if let Some((Constant::Bool(is_true), _)) = constant(cx, cx.tables, lit) {
                    if is_true {
                        span_help_and_lint(
                            cx,
                            ASSERTIONS_ON_CONSTANTS,
                            e.span,
                            "`assert!(true)` will be optimized out by the compiler",
                            "remove it",
                        );
                    } else if !is_debug_assert {
                        span_help_and_lint(
                            cx,
                            ASSERTIONS_ON_CONSTANTS,
                            e.span,
                            "`assert!(false)` should probably be replaced",
                            "use `panic!()` or `unreachable!()`",
                        );
                    }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
                }
            }
        };
        if let Some(debug_assert_span) = is_expn_of(e.span, "debug_assert") {
            if debug_assert_span.from_expansion() {
                return;
            }
            lint_assert_cb(true);
        } else if let Some(assert_span) = is_direct_expn_of(e.span, "assert") {
            if assert_span.from_expansion() {
                return;
            }
            lint_assert_cb(false);
        }
    }
}
