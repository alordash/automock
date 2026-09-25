use crate::common::data_field;
use not_enough_syntax::*;
use std::borrow::BorrowMut;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn normalize_struct_type_references_in_signature(
    mut signature: Signature,
    struct_path: &Path,
) -> Signature {
    let mut normalizer = StructTypeReferencesNormalizer::new(struct_path);
    normalizer.visit_signature_mut(&mut signature);
    return signature;
}

pub(crate) fn normalize_struct_type_references_in_block<T: BorrowMut<Block>>(
    mut block: T,
    struct_path: &Path,
) -> T {
    let mut normalizer = StructTypeReferencesNormalizer::new(struct_path);
    normalizer.visit_block_mut(block.borrow_mut());
    return block;
}

pub(crate) fn normalize_struct_type_references_in_trait_item_fn(
    mut trait_item_fn: TraitItemFn,
    trait_mock_path: &Path,
) -> TraitItemFn {
    let mut normalizer = StructTypeReferencesNormalizer::new(trait_mock_path);
    normalizer.visit_trait_item_fn_mut(&mut trait_item_fn);
    return trait_item_fn;
}

pub(crate) fn normalize_struct_type_references_in_impl_item_fn(
    impl_item_fn: &mut ImplItemFn,
    struct_path: &Path,
) {
    let mut normalizer = StructTypeReferencesNormalizer::new(struct_path);
    normalizer.visit_impl_item_fn_mut(impl_item_fn);
}

pub(crate) fn normalize_in_type(mut ty: Type, struct_path: &Path) -> Type {
    let mut normalizer = StructTypeReferencesNormalizer::new(struct_path);
    normalizer.visit_type_mut(&mut ty);
    return ty;
}

struct StructTypeReferencesNormalizer<'a> {
    struct_path: &'a Path,
    maybe_struct_ident: Option<&'a Ident>,
}

impl<'a> StructTypeReferencesNormalizer<'a> {
    pub fn new(struct_path: &'a Path) -> Self {
        Self {
            struct_path,
            maybe_struct_ident: if struct_path.segments.len() == 1 {
                Some(&struct_path.segments[0].ident)
            } else {
                None
            },
        }
    }

    fn is_struct_path(&self, path: &Path) -> bool {
        if path.segments.len() == 1 && path.segments[0].ident == "Self" {
            return true;
        }

        return path::equal(path, self.struct_path);
    }
}

impl<'a> VisitMut for StructTypeReferencesNormalizer<'a> {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        if let Expr::Path(expr_path) = i
            && self.is_struct_path(&expr_path.path)
        {
            let span = expr_path.span();
            let mut expr_struct = ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: expr_path.path.leading_colon,
                    segments: core::mem::take(&mut expr_path.path.segments),
                },
                brace_token: token::Brace(span),
                fields: punctuated([data_field::new_default_value(span)]),
                dot2_token: None,
                rest: None,
            };

            self.visit_path_mut(&mut expr_struct.path);
            *i = Expr::Struct(expr_struct);
        } else {
            visit_mut::visit_expr_mut(self, i);
        }
    }

    fn visit_expr_struct_mut(&mut self, i: &mut ExprStruct) {
        if self.is_struct_path(&i.path) {
            i.fields.push(data_field::new_default_value(i.span()));
        }

        visit_mut::visit_expr_struct_mut(self, i);
    }

    fn visit_pat_mut(&mut self, i: &mut Pat) {
        match i {
            Pat::Path(pat_path) if self.is_struct_path(&pat_path.path) => {
                let span = pat_path.span();
                let mut pat_struct = PatStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: Path {
                        leading_colon: pat_path.path.leading_colon,
                        segments: core::mem::take(&mut pat_path.path.segments),
                    },
                    brace_token: token::Brace(span),
                    fields: Punctuated::new(),
                    rest: Some(PatRest {
                        attrs: Vec::new(),
                        dot2_token: Token![..](span),
                    }),
                };

                self.visit_path_mut(&mut pat_struct.path);
                *i = Pat::Struct(pat_struct);
            }
            Pat::Ident(pat_ident) => {
                if let Some(struct_ident) = self.maybe_struct_ident
                    && pat_ident.ident == *struct_ident
                {
                    let span = struct_ident.span();
                    let pat_struct = PatStruct {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::from_ident(struct_ident.clone()),
                        brace_token: token::Brace(span),
                        fields: Punctuated::new(),
                        rest: Some(PatRest {
                            attrs: Vec::new(),
                            dot2_token: Token![..](span),
                        }),
                    };
                    *i = Pat::Struct(pat_struct);
                } else {
                    visit_mut::visit_pat_mut(self, i)
                }
            }
            _ => visit_mut::visit_pat_mut(self, i),
        }
    }

    fn visit_pat_struct_mut(&mut self, i: &mut PatStruct) {
        if i.rest.is_none() && self.is_struct_path(&i.path) {
            i.rest = Some(PatRest {
                attrs: Vec::new(),
                dot2_token: Token![..](i.span()),
            })
        }

        visit_mut::visit_pat_struct_mut(self, i);
    }

    fn visit_path_mut(&mut self, i: &mut Path) {
        if i.segments.len() == 1 && i.segments[0].ident == "Self" {
            *i = self.struct_path.clone();
        }

        visit_mut::visit_path_mut(self, i);
    }
}
