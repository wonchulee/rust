use rustc_ast as ast;
use rustc_ast::ptr::P;
use rustc_ast::token::{self, Token};
use rustc_ast::tokenstream::{TokenStream, TokenTree};
use rustc_expand::base::{self, *};
use rustc_span::symbol::{Ident, Symbol};
use rustc_span::Span;

use crate::errors::{CommaExpected, IdentArgsRequired, MissingArguments};

pub fn expand_concat_idents<'cx>(
    cx: &'cx mut ExtCtxt<'_>,
    sp: Span,
    tts: TokenStream,
) -> Box<dyn base::MacResult + 'cx> {
    if tts.is_empty() {
        cx.emit_err(MissingArguments { span: sp });
        return DummyResult::any(sp);
    }

    let mut res_str = String::new();
    for (i, e) in tts.into_trees().enumerate() {
        if i & 1 == 1 {
            match e {
                TokenTree::Token(Token { kind: token::Comma, .. }, _) => {}
                _ => {
                    cx.emit_err(CommaExpected { span: sp });
                    return DummyResult::any(sp);
                }
            }
        } else {
            if let TokenTree::Token(token, _) = e {
                if let Some((ident, _)) = token.ident() {
                    res_str.push_str(ident.name.as_str());
                    continue;
                }
            }

            cx.emit_err(IdentArgsRequired { span: sp });
            return DummyResult::any(sp);
        }
    }

    let ident = Ident::new(Symbol::intern(&res_str), cx.with_call_site_ctxt(sp));

    struct ConcatIdentsResult {
        ident: Ident,
    }

    impl base::MacResult for ConcatIdentsResult {
        fn make_expr(self: Box<Self>) -> Option<P<ast::Expr>> {
            Some(P(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind: ast::ExprKind::Path(None, ast::Path::from_ident(self.ident)),
                span: self.ident.span,
                attrs: ast::AttrVec::new(),
                tokens: None,
            }))
        }

        fn make_ty(self: Box<Self>) -> Option<P<ast::Ty>> {
            Some(P(ast::Ty {
                id: ast::DUMMY_NODE_ID,
                kind: ast::TyKind::Path(None, ast::Path::from_ident(self.ident)),
                span: self.ident.span,
                tokens: None,
            }))
        }
    }

    Box::new(ConcatIdentsResult { ident })
}
