/// Defines number of times the function should have been called.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Times(pub(crate) usize);

/// Allows writing `2.times()`.
pub trait AsTimes: Sized {
    fn times(self) -> Times;
}

impl AsTimes for usize {
    fn times(self) -> Times {
        if self == 0 {
            panic!("Number of expected calls must be greater than 0.")
        }
        Times(self)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn AsTimes_times_Ok() {
        // Arrange
        let raw = 1usize;

        // Act
        let times = raw.times();

        // Assert
        assert_eq!(Times(1), times);
    }
}
