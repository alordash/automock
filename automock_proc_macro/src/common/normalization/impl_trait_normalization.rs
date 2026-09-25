use not_enough_syntax::*;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) struct Result {
    pub ty: Type,
    pub is_impl_trait: bool,
    pub needs_pinning: bool,
}
pub(crate) fn replace_impl_trait_with_box_dyn_trait(mut ty: Type) -> Result {
    let mut is_impl_trait = false;
    let mut needs_pinning = false;
    if let Type::ImplTrait(mut type_impl_trait) = ty {
        let span = type_impl_trait.span();
        needs_pinning = type_impl_trait.bounds.iter().any(|x| {
            if let TypeParamBound::Trait(trait_bound) = x {
                trait_bound
                    .path
                    .segments
                    .last()
                    .is_some_and(|last_segment| last_segment.ident == "Future")
            } else {
                false
            }
        });
        let type_dyn_trait = TypeTraitObject {
            attrs: Vec::new(),
            dyn_token: Some(Token![dyn](span)),
            bounds: core::mem::take(&mut type_impl_trait.bounds),
        };
        let mut new_type = TypePath {
            attrs: Vec::new(),
            qself: None,
            path: path::new_generics(
                span,
                ["Box"],
                [GenericArgument::Type(Type::TraitObject(type_dyn_trait))],
            ),
        };
        if needs_pinning {
            new_type = TypePath {
                attrs: Vec::new(),
                qself: None,
                path: path::new_generics_global(
                    span,
                    ["core", "pin", "Pin"],
                    [GenericArgument::Type(Type::Path(new_type))],
                ),
            };
        }
        ty = Type::Path(new_type);
        is_impl_trait = true;
    }
    let result = Result {
        ty,
        is_impl_trait,
        needs_pinning,
    };
    return result;
}

pub(crate) struct BoxImplParams {
    pub block: Block,
    pub needs_pinning: bool,
}
pub(crate) fn box_impl_trait_return_values(
    BoxImplParams {
        mut block,
        needs_pinning,
    }: BoxImplParams,
) -> Block {
    ImplTraitReturnValueBoxer.visit_block_mut(&mut block);
    if let Some(Stmt::Expr(return_expr, None)) = block.stmts.last_mut() {
        let span = return_expr.span();
        let decoy_expr = Expr::Verbatim(TokenStream::new());
        let source_return_expr = core::mem::replace(return_expr, decoy_expr);
        let box_expr = if needs_pinning {
            Expr::Path(expr::path::new(span, ["Box", "pin"]))
        } else {
            Expr::Path(expr::path::new(span, ["Box", "new"]))
        };
        *return_expr = Expr::Call(expr::call::new(span, box_expr, [source_return_expr]));
    }
    return block;
}

struct ImplTraitReturnValueBoxer;

impl VisitMut for ImplTraitReturnValueBoxer {
    fn visit_expr_return_mut(&mut self, i: &mut ExprReturn) {
        if let Some(return_expr) = i.expr.as_mut() {
            let span = return_expr.span();
            let decoy_expr = Box::new(Expr::Verbatim(TokenStream::new()));
            let source_return_expr = core::mem::replace(return_expr, decoy_expr);
            **return_expr = Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new(span, ["Box", "new"])),
                [*source_return_expr],
            ));
        }

        visit_mut::visit_expr_return_mut(self, i);
    }
}
