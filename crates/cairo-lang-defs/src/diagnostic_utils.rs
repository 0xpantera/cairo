use std::fmt;

use cairo_lang_debug::DebugWithDb;
use cairo_lang_diagnostics::DiagnosticLocation;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextSpan, TextWidth};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};

use crate::db::DefsGroup;

/// A stable location of a real, concrete syntax.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct StableLocation(SyntaxStablePtrId);
impl StableLocation {
    pub fn new(stable_ptr: SyntaxStablePtrId) -> Self {
        Self(stable_ptr)
    }

    pub fn file_id(&self, db: &dyn DefsGroup) -> FileId {
        self.0.file_id(db)
    }

    pub fn from_ast<TNode: TypedSyntaxNode>(db: &dyn SyntaxGroup, node: &TNode) -> Self {
        Self(node.as_syntax_node().stable_ptr(db))
    }

    /// Returns the [SyntaxNode] that corresponds to the [StableLocation].
    pub fn syntax_node(&self, db: &dyn DefsGroup) -> SyntaxNode {
        self.0.lookup(db)
    }

    /// Returns the [SyntaxStablePtrId] of the [StableLocation].
    pub fn stable_ptr(&self) -> SyntaxStablePtrId {
        self.0
    }

    /// Returns the [DiagnosticLocation] that corresponds to the [StableLocation].
    pub fn diagnostic_location(&self, db: &dyn DefsGroup) -> DiagnosticLocation {
        let syntax_node = self.syntax_node(db);
        DiagnosticLocation { file_id: self.file_id(db), span: syntax_node.span_without_trivia(db) }
    }

    /// Returns the [DiagnosticLocation] that corresponds to the [StableLocation].
    pub fn diagnostic_location_until(
        &self,
        db: &dyn DefsGroup,
        until_stable_ptr: SyntaxStablePtrId,
    ) -> DiagnosticLocation {
        let start = self.0.lookup(db).span_start_without_trivia(db);
        let end = until_stable_ptr.lookup(db).span_end_without_trivia(db);
        DiagnosticLocation { file_id: self.0.file_id(db), span: TextSpan { start, end } }
    }

    /// Returns the [DiagnosticLocation] corresponding to a subrange of the [StableLocation],
    /// defined by character offsets relative to the start of the syntax node.
    pub fn diagnostic_location_with_offsets(
        &self,
        db: &dyn DefsGroup,
        start_offset: u32,
        end_offset: u32,
    ) -> DiagnosticLocation {
        let syntax_node = self.0.lookup(db);
        let node_span = syntax_node.span_without_trivia(db);

        let span = TextSpan {
            start: node_span
                .start
                .add_width(TextWidth::new_for_testing(start_offset))
                .min(node_span.end),
            end: node_span
                .start
                .add_width(TextWidth::new_for_testing(end_offset))
                .min(node_span.end),
        };

        DiagnosticLocation { file_id: self.0.file_id(db), span }
    }
}

impl DebugWithDb<dyn DefsGroup> for StableLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &dyn DefsGroup) -> fmt::Result {
        let diag_location = self.diagnostic_location(db);
        diag_location.fmt_location(f, db)
    }
}
