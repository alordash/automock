use crate::common::data_field;
use not_enough_syntax::*;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn modify(mut item_struct: ItemStruct) -> ItemStruct {
    let span = item_struct.span();
    match &mut item_struct.fields {
        Fields::Named(named_fields) => {
            named_fields.named.push(data_field::new_field(span));
            false
        }
        Fields::Unnamed(_) => {
            panic!(
                "Structs with unnamed (tuple) fields can not be mocked, only named fields or no fields at all are supported."
            );
        }
        Fields::Unit => {
            item_struct.fields = Fields::Named(FieldsNamed {
                brace_token: token::Brace(span),
                named: punctuated([data_field::new_field(span)]),
            });
            true
        }
    };

    return item_struct;
}
