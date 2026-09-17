use crate::args::*;
use crate::fn_parameters::*;
use crate::*;

pub struct DynCall<'rs> {
    inner: Box<dyn ICall + 'rs>,
}

impl<'rs> ICall for DynCall<'rs> {
    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        self.inner.get_arg_infos()
    }

    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
        self.inner.get_ptr_to_boxed_tuple_of_refs()
    }
}

impl<'rs> IGenericsInfoProvider for DynCall<'rs> {
    fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
        self.inner.get_generic_parameter_infos()
    }

    fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_generics_type_ids(hasher)
    }

    fn hash_const_values(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_const_values(hasher)
    }
}

impl<'rs> DynCall<'rs> {
    pub(crate) fn new<'a, T: ICall + 'rs>(value: T) -> DynCall<'a> {
        transmute_lifetime!(Self {
            inner: Box::new(value),
        })
    }

    pub fn downcast_ref<T: 'rs>(&self) -> &T {
        let dyn_ref = self.inner.as_ref();
        // SAFETY: for justification refer to module level documentation.
        let t_ref = unsafe { &*(dyn_ref as *const _ as *const T) };
        return t_ref;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use crate::fn_parameters::tests::*;
    use rsubstitute::{AsTimes, Mockable};

    #[test]
    fn new_Ok() {
        // Arrange
        let mut call_mock = StubCall::new();
        call_mock
            .setup()
            .as_ICall()
            .get_arg_infos()
            .returns(Vec::new());

        // Act
        let result = DynCall::new(call_mock);

        // Assert
        let arg_infos = result.inner.get_arg_infos();
        assert!(arg_infos.is_empty());
    }

    #[test]
    fn ICall_get_arg_infos_ForwardsToInner() {
        // Arrange
        let mut call_mock = StubCall::new();
        let arg_infos = vec![
            ArgInfo::new("quo 1", "vadis 1", "veridis 1".to_owned()),
            ArgInfo::new("quo 2", "vadis 2", "veridis 2".to_owned()),
        ];
        call_mock
            .setup()
            .as_ICall()
            .get_arg_infos()
            .returns(arg_infos.clone());

        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        let result = dyn_call.get_arg_infos();

        // Assert
        assert_eq!(result, arg_infos);
        call_mock
            .received()
            .as_ICall()
            .get_arg_infos(1.time())
            .no_other_calls();
    }

    #[test]
    fn ICall_get_ptr_to_boxed_tuple_of_refs_ForwardsToInner() {
        // Arrange
        let mut call_mock = StubCall::new();
        let ptr = 123usize as *mut ();
        call_mock
            .setup()
            .as_ICall()
            .get_ptr_to_boxed_tuple_of_refs()
            .returns(ptr);

        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        let result = dyn_call.get_ptr_to_boxed_tuple_of_refs();

        // Assert
        assert_eq!(result, ptr);
        call_mock
            .received()
            .as_ICall()
            .get_ptr_to_boxed_tuple_of_refs(1.time())
            .no_other_calls();
    }

    #[test]
    fn IGenericsInfoProvider_get_generic_parameter_infos_ForwardsToInner() {
        // Arrange
        let generic_parameter_infos = vec![GenericParameterInfo::Type(GenericTypeInfo {
            name: "quo",
            type_name: "vadis",
        })];
        let mut call_mock = StubCall::new();
        call_mock
            .setup()
            .as_IGenericsInfoProvider()
            .get_generic_parameter_infos()
            .returns(generic_parameter_infos.clone());
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        let result = dyn_call.get_generic_parameter_infos();

        // Assert
        assert_eq!(result, generic_parameter_infos);

        call_mock
            .received()
            .as_IGenericsInfoProvider()
            .get_generic_parameter_infos(1.time())
            .no_other_calls();
    }

    #[test]
    fn IGenericsInfoProvider_hash_generics_type_ids_ForwardsToInner() {
        // Arrange
        let mut generics_hasher = GenericsHasher::new();
        let mut call_mock = StubCall::new();
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        dyn_call.hash_generics_type_ids(&mut generics_hasher);

        // Assert
        call_mock
            .received()
            .as_IGenericsInfoProvider()
            .hash_generics_type_ids(rsubstitute::Arg::ref_eq(&mut generics_hasher), 1.time())
            .no_other_calls();
    }

    #[test]
    fn IGenericsInfoProvider_hash_const_values_ids_ForwardsToInner() {
        // Arrange
        let mut generics_hasher = GenericsHasher::new();
        let mut call_mock = StubCall::new();
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        dyn_call.hash_const_values(&mut generics_hasher);

        // Assert
        call_mock
            .received()
            .as_IGenericsInfoProvider()
            .hash_const_values(rsubstitute::Arg::ref_eq(&mut generics_hasher), 1.time())
            .no_other_calls();
    }
}
