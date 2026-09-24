use crate::args::*;
use crate::fn_parameters::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct FnConfig<'rs, TMock> {
    _phantom_mock: PhantomData<TMock>,
    pub args_checker: DynArgsChecker<'rs>,
    pub return_value_sources: VecDeque<ReturnValueSource<'rs>>,
    pub calls: Vec<Rc<DynCall<'rs>>>,
    #[allow(clippy::type_complexity)]
    pub callback: Option<Rc<RefCell<dyn FnMut(*const (), &DynCall<'rs>)>>>,
    pub call_base: bool,
}

impl<'rs, TMock> FnConfig<'rs, TMock> {
    pub(crate) fn new(args_checker: DynArgsChecker<'rs>) -> Self {
        FnConfig {
            _phantom_mock: PhantomData,
            args_checker,
            return_value_sources: VecDeque::new(),
            calls: Vec::new(),
            callback: None,
            call_base: false,
        }
    }

    pub(crate) fn add_return_value_source(&mut self, return_value: ReturnValueSource<'rs>) {
        self.return_value_sources.push_back(return_value);
    }

    pub(crate) fn add_return_value_sources(
        &mut self,
        return_values: impl IntoIterator<Item = ReturnValueSource<'rs>>,
    ) {
        self.return_value_sources.extend(return_values);
    }

    pub(crate) fn set_callback<TArgRefsTuple, TMockArg>(
        &mut self,
        mut callback: impl FnMut(&TMockArg, TArgRefsTuple) + 'static,
    ) {
        let dyn_callback = move |raw_mock_ptr: *const (), dyn_call: &DynCall<'rs>| {
            let arg_refs_tuple = if size_of::<TArgRefsTuple>() == 0 {
                // SAFETY: target type is ZST, it is safe to initialize it using zeroed memory
                unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
            } else {
                let raw_arg_refs_tuple_ptr = dyn_call.get_ptr_to_boxed_tuple_of_refs();
                let arg_refs_tuple_ptr = raw_arg_refs_tuple_ptr as *mut TArgRefsTuple;

                // SAFETY: both `get_ptr_to_boxed_tuple_of_refs` implementation and `TArgRefsTuple` type
                // are controlled by procedure macro. This guarantees that downcasting from `Box` is safe
                // and won't lead to transmutation between different types.
                let boxed_arg_refs_tuple = unsafe { Box::from_raw(arg_refs_tuple_ptr) };
                *boxed_arg_refs_tuple
            };
            // SAFETY: using pointer instead of reference to untie `TMock` lifetime from `callback`
            // in `FnConfig`. Pointer is passed from `FnData` which casts valid reference to pointer.
            let mock_ref = unsafe {
                let mock_ptr = raw_mock_ptr as *const TMockArg;
                mock_ptr
                    .as_ref()
                    .expect("Pointer to mock in user callback must be not null.")
            };
            callback(mock_ref, arg_refs_tuple)
        };
        self.callback = Some(Rc::new(RefCell::new(dyn_callback)));
    }

    pub(crate) fn register_call(&mut self, call: Rc<DynCall<'rs>>) {
        self.calls.push(call);
    }

    pub(crate) fn check_call(&self, call: &DynCall<'rs>) -> Vec<ArgCheckResult> {
        self.args_checker.check(call)
    }

    pub(crate) fn has_return_value(&self) -> bool {
        self.call_base || self.return_value_sources.front().is_some()
    }

    pub(crate) fn select_next_return_value(
        &mut self,
        call: &DynCall<'rs>,
    ) -> Option<DynReturnValue<'rs>> {
        let return_value_source = self.return_value_sources.front()?;

        return match return_value_source {
            ReturnValueSource::SingleTime(_) => {
                let Some(ReturnValueSource::SingleTime(return_value)) =
                    self.return_value_sources.pop_front()
                else {
                    panic!(
                        "Front return value source must be not empty and single time because it was just checked."
                    )
                };
                Some(return_value)
            }
            ReturnValueSource::Perpetual(perpetual_factory) => {
                let return_value = perpetual_factory();
                Some(return_value)
            }
            ReturnValueSource::Factory(factory) => {
                let dyn_arg_refs_tuple = call.get_dyn_tuple_of_refs();
                let return_value = factory(dyn_arg_refs_tuple);
                return Some(return_value);
            }
        };
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn get_callback(&self) -> Option<Rc<RefCell<dyn FnMut(*const (), &DynCall<'rs>)>>> {
        self.callback.clone()
    }

    pub(crate) fn set_call_base(&mut self) {
        self.call_base = true;
    }

    pub(crate) fn should_call_base(&self) -> bool {
        self.call_base
    }
}
