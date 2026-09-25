use crate::common::models::*;
use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use crate::generation::*;
use crate::preparation::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params<'a> {
    pub attributes: Vec<Attribute>,
    pub unsafety: Option<Token![unsafe]>,
    pub mock_struct_path: Path,
    pub associated_fns: &'a [Ordered<FnInfo>],
    pub static_fns: &'a [Ordered<FnInfo>],
    pub generics: Generics,
    pub mod_ident: &'a Ident,
    pub qualify_call: bool,
}
pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    Params {
        attributes,
        unsafety,
        mock_struct_path,
        associated_fns,
        static_fns,
        generics,
        mod_ident,
        qualify_call,
    }: Params,
) -> ItemImpl {
    let base_fns = if ctx.support_base_calling {
        associated_fns
            .iter()
            .chain(static_fns.iter())
            .map(|ordered| {
                ordered.ref_map(|x| try_extract_base_fn(span, x, Some(mod_ident.clone())))
            })
            .filter_map(|ordered| match ordered.value {
                Some(x) => Some(Ordered::new(ordered.order_number, x)),
                _ => None,
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut fns: Vec<_> = associated_fns
        .iter()
        .map(|ordered| {
            map_fn(
                ctx,
                mock_struct_path.clone(),
                ordered,
                mod_ident.clone(),
                false,
                None,
                qualify_call,
            )
        })
        .chain(static_fns.iter().map(|ordered| {
            map_fn(
                ctx,
                mock_struct_path.clone(),
                ordered,
                mod_ident.clone(),
                true,
                None,
                qualify_call,
            )
        }))
        .collect();
    fns.sort_by_key(|a| a.order_number);
    let items = fns
        .into_iter()
        .chain(base_fns)
        .map(|x| ImplItem::Fn(x.value))
        .collect();
    let result = generate_item_impl(
        attributes,
        unsafety,
        span,
        generics,
        mock_struct_path,
        items,
        None,
    );
    return result;
}

pub(crate) struct ParamsForTrait<'a> {
    pub attributes: Vec<Attribute>,
    pub unsafety: Option<Token![unsafe]>,
    pub mock_struct_path: Path,
    pub constants: &'a [Ordered<ImplItemConst>],
    pub types: &'a [Ordered<ImplItemType>],
    pub associated_fns: &'a [Ordered<FnInfo>],
    pub static_fns: &'a [Ordered<FnInfo>],
    pub merged_generics: Generics,
    pub trait_path: Path,
    pub mod_ident: &'a Ident,
}
pub(crate) struct ResultForTrait {
    pub trait_impl: ItemImpl,
    pub maybe_base_trait_and_fns_impl: Option<(ItemTrait, ItemImpl)>,
}
pub(crate) fn generate_for_trait(
    ctx: &Context,
    span: Span,
    ParamsForTrait {
        attributes,
        unsafety,
        mock_struct_path,
        constants,
        types,
        associated_fns,
        static_fns,
        merged_generics,
        trait_path,
        mod_ident,
    }: ParamsForTrait,
) -> ResultForTrait {
    let maybe_base_trait_and_fns_impl = ctx.support_base_calling.then(|| {
        let base_fns: Vec<_> = associated_fns
            .iter()
            .chain(static_fns.iter())
            .map(|ordered| ordered.ref_map(|x| try_extract_base_fn(span, x, None)))
            .filter_map(|ordered| match ordered.value {
                Some(x) => Some(Ordered::new(ordered.order_number, x)),
                _ => None,
            })
            .collect();
        let fn_generics = TraitItemFn {
            attrs: Vec::new(),
            modifiers: FnModifiers::default(),
            sig: Signature {
                constness: None,
                asyncness: None,
                safety: Safety::Default,
                abi: None,
                fn_token: Token![fn](span),
                ident: Ident::new("__generics", span),
                generics: Generics::default(),
                paren_token: token::Paren(span),
                inputs: Punctuated::new(),
                variadic: None,
                output: ReturnType::Type(
                    Token![->](span),
                    Box::new(generics_phantom_data::new(
                        span,
                        generics_phantom_data::Params {
                            generics: &merged_generics,
                            maybe_argument_types: None,
                        },
                    )),
                ),
            },
            default: Some(Block {
                brace_token: token::Brace(span),
                stmts: vec![Stmt::Expr(
                    Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::new_global(span, ["core", "marker", "PhantomData"]),
                    }),
                    None,
                )],
            }),
            semi_token: None,
        };
        let trait_base_fns: Vec<TraitItemFn> = base_fns
            .iter()
            .zip(associated_fns.iter().chain(static_fns.iter()))
            .map(|(x, fn_info)| {
                let mut signature = x.sig.clone();
                if let Some(FnArg::Receiver(receiver)) = fn_info.source_signature.inputs.first() {
                    signature.inputs[0] = FnArg::Receiver(receiver.clone());
                }
                TraitItemFn {
                    attrs: Vec::new(),
                    modifiers: FnModifiers::default(),
                    sig: signature,
                    default: None,
                    semi_token: Some(Token![;](span)),
                }
            })
            .collect();
        let base_fn_trait = ItemTrait {
            attrs: Vec::new(),
            vis: visibility::pub_super(span),
            modifiers: TraitModifiers::default(),
            unsafety: None,
            trait_token: Token![trait](span),
            ident: format_ident!("__rs_base_{}_{}", mod_ident, path::last_ident(&trait_path)),
            generics: merged_generics.clone(),
            colon_token: None,
            supertraits: punctuated([TypeParamBound::Trait(TraitBound {
                paren_token: None,
                lifetimes: None,
                modifiers: TraitBoundModifiers::default(),
                maybe: None,
                path: trait_path.clone(),
            })]),
            brace_token: token::Brace(span),
            items: core::iter::once(fn_generics)
                .chain(trait_base_fns)
                .map(TraitItem::Fn)
                .collect(),
        };
        let base_fn_items = base_fns
            .into_iter()
            .map(|x| ImplItem::Fn(x.value))
            .collect();
        let base_fns_impl = generate_item_impl(
            Vec::new(),
            None,
            span,
            merged_generics.clone(),
            mock_struct_path.clone(),
            base_fn_items,
            Some(path::from_ident_with_generics(
                base_fn_trait.ident.clone(),
                &base_fn_trait.generics,
            )),
        );
        return (base_fn_trait, base_fns_impl);
    });
    let maybe_base_trait_ident = maybe_base_trait_and_fns_impl
        .as_ref()
        .map(|x| x.0.ident.clone());
    let fns: Vec<_> = associated_fns
        .iter()
        .map(|ordered| {
            map_fn(
                ctx,
                mock_struct_path.clone(),
                ordered,
                mod_ident.clone(),
                false,
                maybe_base_trait_ident.clone(),
                true,
            )
        })
        .chain(static_fns.iter().map(|ordered| {
            map_fn(
                ctx,
                mock_struct_path.clone(),
                ordered,
                mod_ident.clone(),
                true,
                maybe_base_trait_ident.clone(),
                true,
            )
        }))
        .collect();
    let mut ordered_items: Vec<_> = constants
        .iter()
        .map(|ordered| ordered.ref_map(|x| ImplItem::Const(x.clone())))
        .chain(
            types
                .iter()
                .map(|ordered| ordered.ref_map(|x| ImplItem::Type(x.clone()))),
        )
        .chain(fns.into_iter().map(|ordered| ordered.map(ImplItem::Fn)))
        .collect();
    ordered_items.sort_by_key(|a| a.order_number);
    let items = ordered_items.into_iter().map(|x| x.value).collect();
    let trait_impl = generate_item_impl(
        attributes,
        unsafety,
        span,
        merged_generics,
        mock_struct_path,
        items,
        Some(trait_path),
    );
    let result = ResultForTrait {
        trait_impl,
        maybe_base_trait_and_fns_impl,
    };
    return result;
}

fn generate_item_impl(
    mut attributes: Vec<Attribute>,
    unsafety: Option<Token![unsafe]>,
    span: Span,
    generics: Generics,
    mock_struct_path: Path,
    items: Vec<ImplItem>,
    maybe_trait_path: Option<Path>,
) -> ItemImpl {
    attributes.push(attributes::allow_clippy(span, "useless_transmute"));
    let result = ItemImpl {
        attrs: attributes,
        modifiers: ImplModifiers::default(),
        unsafety,
        impl_token: Token![impl](span),
        generics,
        trait_: maybe_trait_path.map(|trait_path| (trait_path, Token![for](span))),
        self_ty: Box::new(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: mock_struct_path,
        })),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

fn map_fn(
    ctx: &Context,
    mock_struct_path: Path,
    ordered_fn_info: &Ordered<FnInfo>,
    mod_ident: Ident,
    is_static: bool,
    maybe_base_trait_ident: Option<Ident>,
    qualify_call: bool,
) -> Ordered<ImplItemFn> {
    ordered_fn_info.ref_map(|fn_info| {
        let span = fn_info.spans.inputs;
        ImplItemFn {
            attrs: fn_info.attributes.clone(),
            vis: fn_info.visibility.clone(),
            modifiers: FnModifiers::default(),
            sig: fn_info.source_signature.clone(),
            block: if is_static {
                static_fn_block::generate(
                    ctx,
                    span,
                    static_fn_block::Params {
                        mock_struct_path,
                        fn_info,
                        base_fn_kind: if fn_info.maybe_base_impl.is_some() {
                            BaseFnKind::Associated(base_fn::get_base_fn_ident(&fn_info.fn_ident))
                        } else {
                            BaseFnKind::None
                        },
                        mod_ident,
                        for_struct: true,
                        maybe_base_trait_ident,
                        use_mod_path_for_call_stmt: true,
                    },
                )
            } else {
                associated_method_block::generate(
                    ctx,
                    span,
                    associated_method_block::Params {
                        mock_struct_path,
                        fn_info,
                        maybe_base_fn_ident: if fn_info.maybe_base_impl.is_some() {
                            Some(base_fn::get_base_fn_ident(&fn_info.fn_ident))
                        } else {
                            None
                        },
                        mod_ident,
                        maybe_base_trait_ident,
                        qualify_call,
                    },
                )
            },
        }
    })
}

fn try_extract_base_fn(
    span: Span,
    fn_info: &FnInfo,
    maybe_mod_ident: Option<Ident>,
) -> Option<ImplItemFn> {
    fn_info.maybe_base_impl.clone().map(|base_impl| {
        base_fn::generate_associated(
            span,
            base_fn::AssociatedParams {
                fn_info,
                base_impl,
                maybe_associated_items_info: None,
                maybe_mod_ident,
            },
        )
    })
}
