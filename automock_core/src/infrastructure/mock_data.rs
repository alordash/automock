use crate::args::*;
use crate::fn_parameters::{GenericParameterInfosFormattingPolicy, fmt_generic_parameter_infos};
use crate::infrastructure::*;
use error_printing::panic_unused_configs_error;
use indexmap::IndexMap;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

// fn name -> fn generics -> mock data
type MockDataMap = IndexMap<String, IndexMap<GenericsHashKey, *const ()>>;

static MOCK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct MockData {
    id: usize,
    map: RwLock<MockDataMap>,
}

pub type SharedMockData = Arc<MockData>;

unsafe impl Send for MockData {}
unsafe impl Sync for MockData {}

impl core::fmt::Debug for MockData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let length = self
            .map
            .read()
            .expect(UNABLE_TO_LOCK_FOR_READING_ERROR)
            .len();
        f.debug_struct("MockData")
            .field("map.len", &length)
            .finish()
    }
}

#[allow(clippy::derivable_impls)]
impl Default for MockData {
    fn default() -> Self {
        Self {
            id: MOCK_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            map: Default::default(),
        }
    }
}

impl MockData {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn verify_calls(&self) {
        let verify_calls_results: Vec<_> = self
            .map
            .read()
            .expect(UNABLE_TO_LOCK_FOR_READING_ERROR)
            .values()
            .flat_map(|y| y.values())
            .cloned()
            .map(
                Self::cast_ptr_to_ref::<
                    'static,
                    (),
                    IRRELEVANT_HAS_RETURN_VALUE,
                    IRRELEVANT_SUPPORTS_BASE_CALLING,
                >,
            )
            .map(|x| x.verify_calls())
            .collect();
        if verify_calls_results
            .iter()
            .all(|x| x.unused_configs.is_empty())
        {
            return;
        }
        panic_unused_configs_error(verify_calls_results);
    }

    pub fn get_fn_data<
        'a,
        TMock: IMockGenericsInfoProvider,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
    >(
        &'_ self,
        owner_type_name: &'static str,
        fn_ident: &'static str,
        generics_hash_key: GenericsHashKey,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, true> {
        let unique_fn_ident = format!("{owner_type_name}_{fn_ident}");
        let generics_info_str = fmt_generic_parameter_infos(
            GenericParameterInfosFormattingPolicy::Perform(&TMock::get_generic_parameter_infos()),
        );
        let owner_name = format!("{owner_type_name}{generics_info_str}");
        let fn_data_ptr = {
            let mut map_write = self.map.write().expect(UNABLE_TO_LOCK_FOR_WRITING_ERROR);
            let fn_data_ptr_ref = map_write
                .entry(unique_fn_ident)
                .or_default()
                .entry(generics_hash_key)
                .or_insert_with(|| {
                    Box::leak(Box::new(FnData::<
                        '_,
                        TMock,
                        HAS_RETURN_VALUE,
                        SUPPORTS_BASE_CALLING,
                        true,
                    >::new(
                        Some(owner_name), fn_ident
                    ))) as *const _ as *const ()
                });
            *fn_data_ptr_ref
        };

        let fn_data_ref = Self::cast_ptr_to_ref(fn_data_ptr);
        return fn_data_ref;
    }

    fn cast_ptr_to_ref<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
    >(
        fn_data_ptr: *const (),
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, true> {
        // SAFETY:
        // `as_ref` - ptr is aligned since it was cast from reference returned from `Box::leak`.
        // Pointed value is a newly created valid `FnData`.
        // Function data is stored behind `Rc<RefCell>` which ensures aliasing rules.
        //
        // `unwrap_unchecked` - pointer was obtained from reference returned from `Box::leak`.
        let fn_data_ref = unsafe {
            (fn_data_ptr as *const _
                as *const FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, true>)
                .as_ref()
                .unwrap_unchecked()
        };

        return fn_data_ref;
    }
}

const IRRELEVANT_HAS_RETURN_VALUE: bool = false;
const IRRELEVANT_SUPPORTS_BASE_CALLING: bool = false;
const IRRELEVANT_PASSES_MOCK_TO_CALLBACK: bool = false;

impl Drop for MockData {
    fn drop(&mut self) {
        for fn_data_ptr in self
            .map
            .read()
            .expect(UNABLE_TO_LOCK_FOR_READING_ERROR)
            .values()
            .flat_map(|x| x.values())
        {
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

const UNABLE_TO_LOCK_FOR_READING_ERROR: &str = "[ERROR] Unable to lock `MockData.map` for reading.";
const UNABLE_TO_LOCK_FOR_WRITING_ERROR: &str = "[ERROR] Unable to lock `MockData.map` for writing.";
