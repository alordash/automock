use std::hash::{DefaultHasher, Hasher};

#[cfg_attr(test, automock::mock)]
#[derive(Clone)]
pub struct GenericsHasher {
    inner: DefaultHasher,
}

#[cfg_attr(test, automock::mock)]
impl GenericsHasher {
    pub fn new() -> Self {
        Self {
            inner: DefaultHasher::new(),
        }
    }
}

#[automock::mock]
impl Hasher for GenericsHasher {
    fn finish(&self) -> u64 {
        self.inner.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.inner.write(bytes);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use automock::Mockable;

    #[test]
    fn finish_Ok() {
        // Arrange
        GenericsHasher::static_setup().new().call_base();
        let mut generics_hasher = GenericsHasher::new();
        generics_hasher.setup().as_Hasher().finish().call_base();

        // Act
        let result = generics_hasher.finish();

        // Assert
        let expected_result = generics_hasher.inner.finish();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn write_Ok() {
        // Arrange
        GenericsHasher::static_setup().new().call_base();
        let mut generics_hasher = GenericsHasher::new();
        generics_hasher
            .setup()
            .as_Hasher()
            .write(automock::Arg::Any)
            .call_base();
        let mut mirror_hasher = generics_hasher.inner.clone();
        let bytes = [1, 2, 3, 4u8];

        // Act
        generics_hasher.write(&bytes);

        // Assert
        mirror_hasher.write(&bytes);
        let expected_hash = mirror_hasher.finish();
        let actual_hash = generics_hasher.inner.finish();
        assert_eq!(actual_hash, expected_hash);
    }

    pub mod utilities {
        use super::*;

        pub fn generics_hasher_mock() -> GenericsHasher {
            GenericsHasher {
                inner: Default::default(),
                __mock_data: Default::default(),
            }
        }
    }
}
