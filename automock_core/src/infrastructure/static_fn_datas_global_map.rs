use crate::infrastructure::{FnData, IMockData};
use std::any::TypeId;
use std::cell::UnsafeCell;
use std::collections::HashMap;

type Map = HashMap<TypeId, HashMap<String, *const ()>>;

// Used for storing static functions' mock data.
#[derive(Default)]
struct StaticFnDatasGlobalMap {
    pub map: UnsafeCell<Map>,
}

impl StaticFnDatasGlobalMap {
    #[allow(clippy::mut_from_ref)]
    fn get_mut_map(&self) -> &mut Map {
        // SAFETY: static functions data is stored in global TLS, which guarantees that there can't
        // be more than one mutable reference to given static function data at the same time.
        // This is why `UnsafeCell` can be safely used here.
        let maybe_map = unsafe { self.map.get().as_mut() };
        // SAFETY: `UnsafeCell::get` can not return null pointer.
        let map = unsafe { maybe_map.unwrap_unchecked() };
        return map;
    }

    fn get_fn_data_ref<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
    >(
        fn_data_raw_ptr: *const (),
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
        // SAFETY: `raw_ptr` is obtained from `Box::<T>::leak`, which means that it is safe to cast
        // the pointer to `T` and treat it as reference.
        let result = unsafe {
            (fn_data_raw_ptr
                as *const FnData<TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false>)
                .as_ref()
                .unwrap_unchecked()
        };
        return result;
    }

    pub fn get_specific_fn_data<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
    >(
        &'_ self,
        maybe_owner_name: Option<&'static str>,
        fn_ident: &'static str,
        for_struct: bool,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
        let type_id = typeid::of::<TMock>();
        let map = self.get_mut_map();
        let fn_key = maybe_owner_name.map_or_else(
            || fn_ident.to_owned(),
            |owner_name| format!("{owner_name}_{fn_ident}"),
        );
        let raw_ptr = map
            .entry(type_id)
            .or_default()
            .entry(fn_key)
            .or_insert_with(|| {
                Box::leak(Box::new(FnData::<
                    TMock,
                    HAS_RETURN_VALUE,
                    SUPPORTS_BASE_CALLING,
                    false,
                >::new(
                    maybe_owner_name, fn_ident, for_struct
                ))) as *mut _ as *const _
            });

        let fn_data_ref =
            Self::get_fn_data_ref::<'a, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING>(*raw_ptr);
        return fn_data_ref;
    }

    pub fn clear_mock_fn_datas<TMock>(&self) {
        let type_id = typeid::of::<TMock>();
        let map = self.get_mut_map();
        map.remove_entry(&type_id);
    }

    pub fn verify_received_nothing_else<TMock>(&self) {
        let type_id = typeid::of::<TMock>();
        let map = self.get_mut_map();
        let Some(fn_datas_raw_ptrs) = map.get(&type_id) else {
            return;
        };
        for fn_data_raw_ptr in fn_datas_raw_ptrs.values() {
            const IRRELEVANT_HAS_RETURN_VALUE: bool = false;
            const IRRELEVANT_SUPPORTS_BASE_CALLING: bool = false;
            let fn_data = Self::get_fn_data_ref::<
                TMock,
                IRRELEVANT_HAS_RETURN_VALUE,
                IRRELEVANT_SUPPORTS_BASE_CALLING,
            >(*fn_data_raw_ptr);
            fn_data.verify_received_nothing_else();
        }
    }
}

// Located in TLS so that tests that mock same function won't overlap if run in parallel.
thread_local! {
    pub static STATIC_FN_DATAS_GLOBAL_MAP: StaticFnDatasGlobalMap = StaticFnDatasGlobalMap {
        map: Default::default(),
    };
}

pub fn get_static_fn_data<
    'a,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
>(
    fn_ident: &'static str,
) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
    let result =
        STATIC_FN_DATAS_GLOBAL_MAP.with(|this| this.get_specific_fn_data(None, fn_ident, false));
    return result;
}

pub fn get_static_fn_data_for_struct<
    'a,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
>(
    owner_name: &'static str,
    fn_ident: &'static str,
) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
    let result = STATIC_FN_DATAS_GLOBAL_MAP
        .with(|this| this.get_specific_fn_data(Some(owner_name), fn_ident, true));
    return result;
}

pub fn clear_static_fn_data<TMock>() {
    STATIC_FN_DATAS_GLOBAL_MAP.with(|this| this.clear_mock_fn_datas::<TMock>());
}

pub fn verify_static_fn_received_nothing_else<TMock>() {
    STATIC_FN_DATAS_GLOBAL_MAP.with(|this| this.verify_received_nothing_else::<TMock>());
}
