use crate::args::*;
use crate::fn_parameters::*;
use crate::*;

pub struct DynCall<'am> {
    inner: Box<dyn ICall + 'am>,
}

impl<'am> ICall for DynCall<'am> {
    fn is_zst(&self) -> bool {
        size_of_val(self.inner.as_ref()) == 0
    }

    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        self.inner.get_arg_infos()
    }

    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
        self.inner.get_ptr_to_boxed_tuple_of_refs()
    }

    fn get_dyn_tuple_of_refs<'a>(&self) -> DynArgRefsTuple<'a> {
        self.inner.get_dyn_tuple_of_refs()
    }
}

impl<'am> IGenericsInfoProvider for DynCall<'am> {
    fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
        self.inner.get_generic_parameter_infos()
    }

    fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_generics_type_ids(hasher)
    }

    fn hash_const_values(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_const_values(hasher)
    }

    fn get_generics_hash_key(&self) -> GenericsHashKey {
        self.inner.get_generics_hash_key()
    }
}

impl<'am> DynCall<'am> {
    pub(crate) fn new<'a, T: ICall + 'am>(value: T) -> DynCall<'a> {
        transmute_lifetime!(Self {
            inner: Box::new(value),
        })
    }

    pub fn downcast_to<T: 'am>(&self) -> &T {
        let dyn_ref = self.inner.as_ref();
        // SAFETY: for justification refer to module level documentation.
        let t_ref = unsafe { &*(dyn_ref as *const _ as *const T) };
        return t_ref;
    }

    pub(crate) fn downcast_into<T>(self) -> T {
        let dyn_ptr = Box::leak(self.inner) as *mut _;
        let dyn_fat_ptr: FatPointer = unsafe { core::mem::transmute(dyn_ptr) };
        let t_ptr = dyn_fat_ptr.data_pointer as *mut T;
        let t_box: Box<T> = unsafe { Box::from_raw(t_ptr) };
        let t = *t_box;
        return t;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use crate::fn_parameters::dyn_arg_refs_tuple::tests::utilities::*;
    use crate::fn_parameters::i_call::tests::utilities::*;
    use automock::{AsTimes, Mockable};

    #[test]
    fn new_Ok() {
        // Arrange
        let mut call_mock = CallMock::new();
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
    fn downcast_to_Ok() {
        // Arrange
        let call_mock = CallMock::new();
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        let result: &CallMock = dyn_call.downcast_to();

        // Assert
        assert_eq!(result.id(), call_mock.id());
    }

    #[test]
    fn downcast_into_Ok() {
        // Arrange
        let call_mock = CallMock::new();
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        let result: CallMock = dyn_call.downcast_into();

        // Assert
        assert_eq!(result.id(), call_mock.id());
    }

    #[test]
    fn ICall_get_arg_infos_ForwardsToInner() {
        // Arrange
        let mut call_mock = CallMock::new();
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
        let mut call_mock = CallMock::new();
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
    fn ICall_get_dyn_tuple_of_refs_ForwardsToInner() {
        // Arrange
        let mut call_mock = CallMock::new();
        let dyn_arg_refs_tuple = dyn_arg_refs_tuple_mock();
        let dyn_arg_refs_tuple_id = dyn_arg_refs_tuple.id();
        call_mock
            .setup()
            .as_ICall()
            .get_dyn_tuple_of_refs()
            .returns(dyn_arg_refs_tuple);

        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        let result = dyn_call.get_dyn_tuple_of_refs();

        // Assert
        assert_eq!(result.id(), dyn_arg_refs_tuple_id);
        call_mock
            .received()
            .as_ICall()
            .get_dyn_tuple_of_refs(1.time())
            .no_other_calls();
    }

    #[test]
    fn IGenericsInfoProvider_get_generic_parameter_infos_ForwardsToInner() {
        // Arrange
        let generic_parameter_infos = vec![GenericParameterInfo::Type(GenericTypeInfo {
            name: "quo",
            type_name: "vadis",
        })];
        let mut call_mock = CallMock::new();
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
        GenericsHasher::static_setup()
            .as_Default()
            .default()
            .call_base();
        let mut generics_hasher = GenericsHasher::default();
        let mut call_mock = CallMock::new();
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        dyn_call.hash_generics_type_ids(&mut generics_hasher);

        // Assert
        call_mock
            .received()
            .as_IGenericsInfoProvider()
            .hash_generics_type_ids(automock::Arg::ref_eq(&mut generics_hasher), 1.time())
            .no_other_calls();
    }

    #[test]
    fn IGenericsInfoProvider_hash_const_values_ids_ForwardsToInner() {
        // Arrange
        GenericsHasher::static_setup()
            .as_Default()
            .default()
            .call_base();
        let mut generics_hasher = GenericsHasher::default();
        let mut call_mock = CallMock::new();
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        dyn_call.hash_const_values(&mut generics_hasher);

        // Assert
        call_mock
            .received()
            .as_IGenericsInfoProvider()
            .hash_const_values(automock::Arg::ref_eq(&mut generics_hasher), 1.time())
            .no_other_calls();
    }

    #[test]
    fn IGenericsInfoProvider_get_generics_hash_key_ForwardsToInner() {
        // Arrange
        let mut call_mock = CallMock::new();
        let generics_hash_key = GenericsHashKey(5);
        call_mock
            .setup()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key()
            .returns(generics_hash_key);
        let dyn_call = DynCall::new(call_mock.clone());

        // Act
        let result = dyn_call.get_generics_hash_key();

        // Assert
        assert_eq!(result, generics_hash_key);

        call_mock
            .received()
            .as_IGenericsInfoProvider()
            .get_generics_hash_key(automock::Times::Once)
            .no_other_calls();
    }
}
