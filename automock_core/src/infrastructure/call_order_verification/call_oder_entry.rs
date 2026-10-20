#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CallOrderEntry {
    pub call_order_number: usize,
    pub formatted_string: String,
}