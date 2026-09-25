use super::models::*;
use crate::constants;

pub(crate) fn create_for_mock_macro(proc_macro_attribute: proc_macro::TokenStream) -> Context {
    if proc_macro_attribute.is_empty() {
        return Default::default();
    }

    let proc_macro_attribute_str = proc_macro_attribute.to_string();
    let parameters: Vec<_> = proc_macro_attribute_str
        .split(',')
        .map(|x| x.trim())
        .collect();

    let support_base_calling = support_base_calling_from_parameters(&parameters);

    let ctx = Context {
        support_base_calling,
    };
    return ctx;
}

fn support_base_calling_from_parameters(parameters: &[&str]) -> bool {
    return parameters.contains(&constants::SUPPORT_BASE_PARAMETER);
}
