use crate::args::{GenericParameterInfo, GenericsHashKey, GenericsHasher};
use std::any::TypeId;
use std::hash::{Hash, Hasher};

#[doc(hidden)]
pub trait IGenericsInfoProvider {
    fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
        Vec::new()
    }

    fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}

    fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}

    fn get_generics_hash_key(&self) -> GenericsHashKey {
        let mut hasher = GenericsHasher::new();
        self.hash_generics_type_ids(&mut hasher);
        self.hash_const_values(&mut hasher);
        let generics_hash_key = GenericsHashKey(hasher.finish());
        return generics_hash_key;
    }
}

// Helper method for clearer `IGenericsInfoProvider::hash_generics_type_ids` auto-generated implementation.
#[doc(hidden)]
pub fn tid<T: ?Sized>() -> TypeId {
    typeid::of::<T>()
}

// Helper method for calculating hash in `IGenericsInfoProvider::hash_consts_values` of any sized
// const value (passed as const parameter) by using type's raw bytes. Not calling `t.hash` because
// `T` is not guaranteed to implement `Hash`.
// This approach anticipates adt_const_params feature:
// https://doc.rust-lang.org/beta/unstable-book/language-features/adt-const-params.html
#[doc(hidden)]
pub fn const_hash<T: Sized + 'static>(t: &T, hasher: &mut GenericsHasher) {
    let t_size = size_of::<T>();
    let t_ptr = t as *const _ as *const u8;
    unsafe {
        let t_slice = std::slice::from_raw_parts(t_ptr, t_size);
        t_slice.hash(hasher);
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use crate::args::generics_hasher::tests::utilities::*;
    use automock::Mockable;
    use utilities::*;

    #[test]
    fn get_generic_parameter_infos_ReturnsEmptyVec() {
        // Arrange
        let generics_info_provider = DefaultGenericsInfoProvider::new();

        // Act
        let result = generics_info_provider.get_generic_parameter_infos();

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn hash_generics_type_ids_DoesNothing() {
        // Arrange
        let generics_info_provider = DefaultGenericsInfoProvider::new();
        let mut hasher = generics_hasher_mock();

        // Act
        generics_info_provider.hash_generics_type_ids(&mut hasher);

        // Assert
        hasher.received().no_other_calls();
    }

    #[test]
    fn hash_const_values_ids_DoesNothing() {
        // Arrange
        let generics_info_provider = DefaultGenericsInfoProvider::new();
        let mut hasher = generics_hasher_mock();

        // Act
        generics_info_provider.hash_const_values(&mut hasher);

        // Assert
        hasher.received().no_other_calls();
    }

    #[test]
    fn get_generics_hash_key_Ok() {
        // Arrange
        let mut generics_info_provider = GenericsInfoProviderWithoutGetGenericsHashKey::new();
        let generics_type_ids_bytes: &[u8] = &[1, 11u8];
        let const_values_bytes: &[u8] = &[2, 22u8];
        generics_info_provider
            .setup()
            .as_IGenericsInfoProvider()
            .hash_generics_type_ids(automock::Arg::Any)
            .does(|_, (hasher,)| hasher.write(generics_type_ids_bytes))
            .hash_const_values(automock::Arg::Any)
            .does(|_, (hasher,)| hasher.write(const_values_bytes));

        let mut generics_hasher_mock = generics_hasher_mock();
        GenericsHasher::static_setup()
            .new()
            .returns(generics_hasher_mock.clone());
        let hash = 5;
        generics_hasher_mock
            .setup()
            .as_Hasher()
            .finish()
            .returns(hash);

        // Act
        let result = generics_info_provider.get_generics_hash_key();

        // Assert
        assert_eq!(result.0, hash);
        // TODO - UB from dangling reference - use `Arg::is` that checks mock object id instead
        generics_info_provider
            .received()
            .as_IGenericsInfoProvider()
            .hash_generics_type_ids(
                automock::Arg::ref_eq(&mut generics_hasher_mock as &mut GenericsHasher),
                automock::Times::Once,
            );
        generics_info_provider
            .received()
            .as_IGenericsInfoProvider()
            .hash_const_values(
                automock::Arg::ref_eq(&mut generics_hasher_mock as &mut GenericsHasher),
                automock::Times::Once,
            )
            .no_other_calls();

        GenericsHasher::static_received()
            .new(automock::Times::Once)
            .no_other_calls();

        generics_hasher_mock
            .received()
            .as_Hasher()
            .write(
                automock::Arg::ref_eq(generics_type_ids_bytes),
                automock::Times::Once,
            )
            .write(
                automock::Arg::ref_eq(const_values_bytes),
                automock::Times::Once,
            )
            .finish(automock::Times::Once)
            .no_other_calls();
    }

    mod utilities {
        use super::*;

        #[automock::mock]
        pub struct DefaultGenericsInfoProvider;
        impl DefaultGenericsInfoProvider {
            pub fn new() -> Self {
                Self {
                    __mock_data: Default::default(),
                }
            }
        }
        impl IGenericsInfoProvider for DefaultGenericsInfoProvider {}

        #[automock::mock]
        pub struct GenericsInfoProviderWithoutGetGenericsHashKey;
        impl GenericsInfoProviderWithoutGetGenericsHashKey {
            pub fn new() -> Self {
                Self {
                    __mock_data: Default::default(),
                }
            }
        }
        #[automock::mock]
        impl IGenericsInfoProvider for GenericsInfoProviderWithoutGetGenericsHashKey {
            fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}

            fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
        }
    }
}
