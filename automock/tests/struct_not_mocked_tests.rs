use automock::*;

#[mock]
struct Structure {
    pub v: i32,
}

#[mock(base)]
impl Structure {
    pub fn new(v: i32) -> Self {
        Self { v }
    }
}

impl Structure {
    pub fn get(&self) -> i32 {
        self.v
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn get_Ok() {
        // Arrange
        let value = 10;
        let mock = Structure::new(value);

        // Act
        let result = mock.get();

        // Assert
        assert_eq!(result, value);
    }
}
