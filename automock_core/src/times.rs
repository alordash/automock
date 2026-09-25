/// Defines number of times the function should have been called.
#[derive(PartialEq, Debug)]
pub enum Times {
    Never,
    Once,
    Exactly(usize),
    #[deprecated = "Expecting varying number of calls is not recommended, as it may lead to inconsistent test run results. Determine concrete number of expected calls."]
    Any,
}

impl Times {
    pub fn matches(&self, count: usize) -> bool {
        match self {
            Times::Never => count == 0,
            Times::Once => count == 1,
            Times::Exactly(exact_count) => count == *exact_count,
            #[allow(deprecated)]
            Times::Any => true,
        }
    }
}

/// Syntactic sugar for writing `1.time()` or `2.times()`.
pub trait AsTimes: Sized {
    fn time(self) -> Times {
        Self::times(self)
    }

    fn times(self) -> Times;
}

impl AsTimes for usize {
    fn times(self) -> Times {
        Times::Exactly(self)
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
        assert_eq!(Times::Exactly(1), times);
    }
}
