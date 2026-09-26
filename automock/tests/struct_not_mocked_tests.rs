use automock::*;

#[mock]
struct Struct {
    pub v: i32,
}

#[mock(base)]
impl Struct {
    pub fn new(v: i32) -> Self {
        Self { v }
    }
}

impl Struct {
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
        Struct::static_setup().new(Arg::Any).call_base();
        let mock = Struct::new(value);

        // Act
        let result = mock.get();

        // Assert
        assert_eq!(result, value);
    }
}
