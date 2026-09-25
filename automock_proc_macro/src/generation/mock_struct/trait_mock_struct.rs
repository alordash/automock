use crate::common::models::*;
use crate::common::*;
use crate::generation::base_fn;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_struct::common::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use crate::generation::trait_info::models::*;
use crate::preparation::models::*;
use crate::preparation::r#trait::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::*;

pub(crate) struct Params<'a> {
    pub mock_struct_ident: Ident,
    pub trait_info: &'a TraitInfo,
    pub generics_for_impl: Generics,
    pub mod_ident: &'a Ident,
    pub maybe_associated_controls: Option<AssociatedControls>,
    pub maybe_static_controls: Option<StaticControls>,
}

pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    Params {
        mock_struct_ident,
        trait_info,
        generics_for_impl,
        mod_ident,
        maybe_associated_controls,
        maybe_static_controls,
    }: Params,
) -> TraitMockStruct {
    let path =
        path::from_ident_with_generics(mock_struct_ident.clone(), &trait_info.merged_generics);
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: mock_struct_ident,
        generics: trait_info.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([
                generics_field::new_field(span, &trait_info.merged_generics, None),
                data_field::new_field(span),
            ]),
        }),
        semi_token: None,
    };
    let clone_impl = clone_impl::generate(
        span,
        generics_for_impl.clone(),
        path.clone(),
        &item_struct.fields,
    );
    let trait_impl = generate_trait_impl(
        ctx,
        span,
        trait_info,
        generics_for_impl.clone(),
        path.clone(),
        mod_ident,
    );
    let inner_impl = generate_inner_impl(
        ctx,
        span,
        trait_info,
        generics_for_impl,
        path,
        mod_ident,
        &maybe_associated_controls,
        &maybe_static_controls,
    );

    let result = TraitMockStruct {
        item_struct,
        clone_impl,
        trait_impl,
        inner_impl,
        maybe_associated_controls,
        maybe_static_controls,
    };
    return result;
}

fn generate_trait_impl(
    ctx: &Context,
    span: Span,
    trait_info: &TraitInfo,
    generics_for_impl: Generics,
    mock_struct_path: Path,
    mod_ident: &Ident,
) -> ItemImpl {
    let mut items_with_order: Vec<_> = trait_info
        .constants
        .iter()
        .map(map_const)
        .chain(trait_info.assoc_types.iter().map(map_assoc_type))
        .chain(
            trait_info
                .associated_fns
                .iter()
                .map(|x| map_fn(ctx, mock_struct_path.clone(), x, mod_ident.clone(), false)),
        )
        .chain(
            trait_info
                .static_fns
                .iter()
                .map(|x| map_fn(ctx, mock_struct_path.clone(), x, mod_ident.clone(), true)),
        )
        .collect();
    items_with_order.sort_by_key(|a| a.order_number);

    let items = items_with_order.into_iter().map(|x| x.value).collect();

    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: trait_info.unsafety,
        impl_token: Token![impl](span),
        generics: generics_for_impl,
        trait_: Some((trait_info.path.clone(), Token![for](span))),
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

fn map_const(ordered_const: &Ordered<TraitItemConstSyntax>) -> Ordered<ImplItem> {
    ordered_const.ref_map(|x| {
        let span = x.corresponding_generic_param_path.span();
        ImplItem::Const(ImplItemConst {
            attrs: x.item.attrs.clone(),
            vis: Visibility::Inherited,
            modifiers: ConstModifiers::default(),
            const_token: x.item.const_token,
            ident: x.item.ident.clone(),
            generics: x.item.generics.clone(),
            colon_token: x.item.colon_token,
            ty: x.item.ty.clone(),
            eq_token: Token![=](span),
            expr: Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: x.corresponding_generic_param_path.clone(),
            }),
            semi_token: Token![;](span),
        })
    })
}

fn map_assoc_type(ordered_assoc_type: &Ordered<TraitItemTypeSyntax>) -> Ordered<ImplItem> {
    ordered_assoc_type.ref_map(|x| {
        let span = x.corresponding_generic_param_path.span();
        ImplItem::Type(ImplItemType {
            attrs: x.item.attrs.clone(),
            vis: Visibility::Inherited,
            modifiers: TypeModifiers::default(),
            type_token: x.item.type_token,
            ident: x.item.ident.clone(),
            generics: x.item.generics.clone(),
            eq_token: Token![=](span),
            ty: Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: x.corresponding_generic_param_path.clone(),
            }),
            semi_token: Token![;](span),
        })
    })
}

fn map_fn(
    ctx: &Context,
    mock_struct_path: Path,
    ordered_fn_info: &Ordered<FnInfo>,
    mod_ident: Ident,
    is_static: bool,
) -> Ordered<ImplItem> {
    ordered_fn_info.ref_map(|fn_info| {
        let span = fn_info.spans.inputs;
        ImplItem::Fn(ImplItemFn {
            attrs: fn_info.attributes.clone(),
            vis: Visibility::Inherited,
            modifiers: FnModifiers::default(),
            sig: *fn_info.signature.clone(),
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
                        for_struct: false,
                        maybe_base_trait_ident: None,
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
                        maybe_base_trait_ident: None,
                        qualify_call: false,
                    },
                )
            },
        })
    })
}

#[allow(clippy::too_many_arguments)]
fn generate_inner_impl(
    ctx: &Context,
    span: Span,
    trait_info: &TraitInfo,
    generics_for_impl: Generics,
    mock_struct_path: Path,
    mod_ident: &Ident,
    maybe_associated_controls: &Option<AssociatedControls>,
    maybe_static_controls: &Option<StaticControls>,
) -> ItemImpl {
    let mock_struct_fn_new = mock_struct_fn_new::new(span);
    let associated_controls_creation_fns =
        maybe_associated_controls
            .as_ref()
            .map(|associated_controls| {
                [
                    control_creation_fn::generate_associated(
                        span,
                        associated_controls.setup_struct.path.clone(),
                        ControlType::Setup,
                    ),
                    control_creation_fn::generate_associated(
                        span,
                        associated_controls.received_struct.path.clone(),
                        ControlType::Received,
                    ),
                ]
            });
    let static_controls_creation_fns = maybe_static_controls.as_ref().map(|static_controls| {
        [
            control_creation_fn::generate_static(
                span,
                static_controls.static_setup_struct.path.clone(),
                StaticControlType::Setup {
                    mock_generic_argument: Box::new(GenericArgument::Type(Type::Path(TypePath {
                        attrs: Vec::new(),
                        qself: None,
                        path: mock_struct_path.clone(),
                    }))),
                },
            ),
            control_creation_fn::generate_static(
                span,
                static_controls.static_received_struct.path.clone(),
                StaticControlType::Received,
            ),
        ]
    });
    let base_fns = if ctx.support_base_calling {
        Some(
            trait_info
                .associated_fns
                .iter()
                .chain(trait_info.static_fns.iter())
                .filter_map(|fn_info| try_extract_base_fn(span, trait_info, fn_info, mod_ident)),
        )
    } else {
        None
    };
    let items = core::iter::once(mock_struct_fn_new)
        .chain(associated_controls_creation_fns.into_iter().flatten())
        .chain(static_controls_creation_fns.into_iter().flatten())
        .chain(base_fns.into_iter().flatten())
        .map(ImplItem::Fn)
        .collect();

    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics: generics_for_impl,
        trait_: None,
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

fn try_extract_base_fn(
    span: Span,
    trait_info: &TraitInfo,
    fn_info: &FnInfo,
    mod_ident: &Ident,
) -> Option<ImplItemFn> {
    fn_info.maybe_base_impl.clone().map(|base_impl| {
        base_fn::generate_associated(
            span,
            base_fn::AssociatedParams {
                fn_info,
                base_impl,
                maybe_associated_items_info: Some(&trait_info.associated_items_info),
                maybe_mod_ident: Some(mod_ident.clone()),
            },
        )
    })
}
