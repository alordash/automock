use crate::args::*;
use crate::infrastructure::*;
use std::sync::{Arc, RwLock};

pub trait ISharedMockData {
    fn get_shared_fn_data<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ self,
        owner_name: &'static str,
        fn_ident: &'static str,
        generics_hash_key: GenericsHashKey,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>;
}

pub type SharedMockData = Arc<RwLock<MockData>>;

impl ISharedMockData for SharedMockData {
    fn get_shared_fn_data<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ self,
        owner_name: &'static str,
        fn_ident: &'static str,
        generics_hash_key: GenericsHashKey,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        let unique_fn_ident = format!("{owner_name}_{fn_ident}");
        self.write()
            .expect(UNABLE_TO_LOCK_FOR_WRITING_ERROR)
            .get_or_create_fn_data(
                Some(owner_name),
                unique_fn_ident,
                fn_ident,
                generics_hash_key,
                false,
            )
    }
}

impl IMockData for SharedMockData {
    fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
        self.read()
            .expect(UNABLE_TO_LOCK_FOR_READING_ERROR)
            .get_received_nothing_else_error_msgs()
    }
}

const UNABLE_TO_LOCK_FOR_WRITING_ERROR: &str = "Unable to lock SharedMockData for writing.";
const UNABLE_TO_LOCK_FOR_READING_ERROR: &str = "Unable to lock SharedMockData for reading.";
