use crate::args::*;
use crate::infrastructure::*;
use indexmap::IndexMap;
use std::fmt::Formatter;

// Two layer map: fn name + fn generics
type Map = IndexMap<String, IndexMap<GenericsHashKey, *const ()>>;

pub struct MockData {
    map: Map,
}

unsafe impl Send for MockData {}
unsafe impl Sync for MockData {}

impl core::fmt::Debug for MockData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockData")
            .field("map.len", &self.map.len())
            .finish()
    }
}

#[allow(clippy::derivable_impls)]
impl Default for MockData {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl MockData {
    pub(crate) fn get_or_create_fn_data<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ mut self,
        maybe_owner_name: Option<&'static str>,
        unique_fn_ident: String, // for trait fns
        fn_ident: &'static str,
        generics_hash_key: GenericsHashKey,
        for_struct: bool,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        let fn_data_ptr = self
            .map
            .entry(unique_fn_ident)
            .or_default()
            .entry(generics_hash_key)
            .or_insert_with(|| {
                Box::leak(Box::new(FnData::<
                    '_,
                    TMock,
                    HAS_RETURN_VALUE,
                    SUPPORTS_BASE_CALLING,
                    PASSES_MOCK_TO_CALLBACK,
                >::new(
                    maybe_owner_name, fn_ident, for_struct
                ))) as *const _ as *const ()
            });

        let fn_data_ref = Self::cast_ptr_to_ref(*fn_data_ptr);
        return fn_data_ref;
    }

    fn cast_ptr_to_ref<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        fn_data_ptr: *const (),
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        // SAFETY:
        // `as_ref` - ptr is aligned since it was cast from reference returned from `Box::leak`.
        // Pointed value is a newly created valid `FnData`.
        // Function data is stored behind `Rc<RefCell>` which ensures aliasing rules.
        //
        // `unwrap_unchecked` - pointer was obtained from reference returned from `Box::leak`.
        let fn_data_ref = unsafe {
            (fn_data_ptr as *const _
                as *const FnData<
                    'static,
                    TMock,
                    HAS_RETURN_VALUE,
                    SUPPORTS_BASE_CALLING,
                    PASSES_MOCK_TO_CALLBACK,
                >)
                .as_ref()
                .unwrap_unchecked()
        };

        return fn_data_ref;
    }
}

const IRRELEVANT_HAS_RETURN_VALUE: bool = false;
const IRRELEVANT_SUPPORTS_BASE_CALLING: bool = false;
const IRRELEVANT_PASSES_MOCK_TO_CALLBACK: bool = false;

impl IMockData for MockData {
    fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
        let result = self
            .map
            .values()
            .flat_map(|y| y.values())
            .cloned()
            .map(
                Self::cast_ptr_to_ref::<
                    'static,
                    (),
                    IRRELEVANT_HAS_RETURN_VALUE,
                    IRRELEVANT_SUPPORTS_BASE_CALLING,
                    IRRELEVANT_PASSES_MOCK_TO_CALLBACK,
                >,
            )
            .map(FnData::get_unexpected_calls_error_msgs)
            .collect();
        return result;
    }
}

impl Drop for MockData {
    fn drop(&mut self) {
        for fn_data_ptr in self.map.values().flat_map(|x| x.values()) {
            let boxed_fn_data = unsafe {
                Box::from_raw(
                    (*fn_data_ptr) as *const _
                        as *mut FnData<
                            'static,
                            (),
                            IRRELEVANT_HAS_RETURN_VALUE,
                            IRRELEVANT_SUPPORTS_BASE_CALLING,
                            IRRELEVANT_PASSES_MOCK_TO_CALLBACK,
                        >,
                )
            };
            drop(boxed_fn_data);
        }
    }
}
