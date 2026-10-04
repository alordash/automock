use super::models::*;
use crate::preparation::r#fn::models::*;
use crate::preparation::r#fn::*;
use crate::preparation::models::*;
use not_enough_syntax::*;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::*;

pub(crate) struct Params {
    pub attributes: Vec<Attribute>,
    pub merged_generics: Generics,
    pub target_type: Box<Type>,
    pub trait_path: Path,
    pub impl_items: Vec<ImplItem>,
}

pub(crate) fn prepare(
    Params {
        attributes,
        merged_generics,
        target_type,
        trait_path,
        impl_items,
    }: Params,
) -> ImplTraitForStructSyntax {
    let trait_ident = ident::combine_path_segments(&trait_path);
    let target_path = prase_target_type(&target_type);
    let split_items = split_items(impl_items);
    let impl_struct_syntax_as_fn_owner = ImplTraitForStructSyntaxAsFnOwner {
        trait_ident: &trait_ident,
        struct_ident: path::last_ident(&target_path),
        generics: &merged_generics,
    };
    let static_fns = split_items
        .static_fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| {
                map_impl_item_fn_to_fn_syntax(x, &impl_struct_syntax_as_fn_owner, &target_path)
            })
        })
        .collect();
    let associated_fns = split_items
        .associated_fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| {
                map_impl_item_fn_to_fn_syntax(x, &impl_struct_syntax_as_fn_owner, &target_path)
            })
        })
        .collect();
    let split_generics = item_impl::split_generics(&merged_generics, &trait_path, &target_type);
    let result = ImplTraitForStructSyntax {
        attributes,
        target_path,
        trait_ident,
        trait_path,
        merged_generics,
        trait_simple_generics: split_generics.trait_generics,
        as_trait_where_predicates: split_generics.trait_where_predicates,
        target_simple_generics: split_generics.target_generics,
        constants: split_items.constants,
        types: split_items.types,
        static_fns,
        associated_fns,
    };
    return result;
}

#[derive(Default)]
struct SplitItems {
    pub constants: Vec<Ordered<ImplItemConst>>,
    pub types: Vec<Ordered<ImplItemType>>,
    pub static_fns: Vec<Ordered<ImplItemFn>>,
    pub associated_fns: Vec<Ordered<ImplItemFn>>,
}
fn split_items(items: Vec<ImplItem>) -> SplitItems {
    let mut split_items = SplitItems::default();
    for (order_number, item) in items.into_iter().enumerate() {
        match item {
            ImplItem::Const(impl_item_const) => split_items
                .constants
                .push(Ordered::new(order_number, impl_item_const)),
            ImplItem::Fn(impl_item_fn) => {
                if signature::is_associated(&impl_item_fn.sig) {
                    split_items
                        .associated_fns
                        .push(Ordered::new(order_number, impl_item_fn))
                } else {
                    split_items
                        .static_fns
                        .push(Ordered::new(order_number, impl_item_fn))
                }
            }
            ImplItem::Type(ty) => split_items.types.push(Ordered::new(order_number, ty)),
            ImplItem::Macro(_) => panic!("Macro invocations inside impl blocks are not supported"),
            _ => panic!("Unexpected impl item: {}", item.to_token_stream()),
        }
    }

    return split_items;
}

fn map_impl_item_fn_to_fn_syntax(
    impl_item_fn: ImplItemFn,
    impl_struct_syntax_as_fn_owner: &ImplTraitForStructSyntaxAsFnOwner,
    target_path: &Path,
) -> FnSyntax {
    let result = fn_syntax::prepare(fn_syntax::Params {
        attributes: impl_item_fn.attrs,
        visibility: impl_item_fn.vis,
        signature: impl_item_fn.sig,
        maybe_base_impl: Some(Box::new(impl_item_fn.block)),
        maybe_owner: Some(impl_struct_syntax_as_fn_owner),
        maybe_target_path: Some(target_path),
    });
    return result;
}

fn prase_target_type(target_type: &Type) -> Path {
    let Type::Path(result) = target_type else {
        panic!("Can mock only `impl`s of structs.");
    };
    if result.qself.is_some() {
        panic!("Can not mock structs qualified with self-type.");
    }

    return result.path.clone();
}

struct ImplTraitForStructSyntaxAsFnOwner<'a> {
    pub trait_ident: &'a Ident,
    pub struct_ident: &'a Ident,
    pub generics: &'a Generics,
}
impl<'a> IFnOwner for ImplTraitForStructSyntaxAsFnOwner<'a> {
    fn maybe_ident(&self) -> Option<&Ident> {
        Some(self.trait_ident)
    }

    fn format_name(&self) -> String {
        format!("<{} as {}>", self.struct_ident, self.trait_ident)
    }

    fn generics(&self) -> &Generics {
        self.generics
    }
}
