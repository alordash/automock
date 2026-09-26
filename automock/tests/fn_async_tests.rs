use automock::*;

const DEFAULT_RESULT: i32 = 123;

#[mock]
async fn nothing() {}

#[mock]
async fn input(_: i32) {}

#[mock]
async fn output() -> i32 {
    unreachable!("base is not mocked")
}

#[mock]
async fn input_output(_: i32) -> i32 {
    unreachable!("base is not mocked")
}

#[mock]
async fn dependency() {}

#[mock(base)]
async fn nothing_base() {
    dependency().await
}

#[mock(base)]
async fn input_base(_: i32) {
    dependency().await
}

#[mock(base)]
async fn output_base() -> i32 {
    dependency().await;
    DEFAULT_RESULT
}

#[mock(base)]
async fn input_output_base(_: i32) -> i32 {
    dependency().await;
    DEFAULT_RESULT
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    async fn nothing_Ok() {
        // Arrange
        // Act
        nothing().await;

        // Assert
        nothing::received(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn input_Ok() {
        // Arrange
        let value = 1;

        // Act
        input(value).await;

        // Assert
        input::received(value, Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn output_Ok() {
        // Arrange
        let result = 2;
        output::setup().returns(result);

        // Act
        let actual_result = output().await;

        // Assert
        assert_eq!(result, actual_result);
        output::received(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn input_output_Ok() {
        // Arrange
        let value = 1;
        let result = 2;
        input_output::setup(Arg::Any).returns(result);

        // Act
        let actual_result = input_output(value).await;

        // Assert
        assert_eq!(result, actual_result);
        input_output::received(value, Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn nothing_base_Ok() {
        // Arrange
        nothing_base::setup().call_base();

        // Act
        nothing_base().await;

        // Assert
        nothing_base::received_nothing();
        dependency::received(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn input_base_Ok() {
        // Arrange
        let value = 1;
        input_base::setup(Arg::Any).call_base();

        // Act
        input_base(value).await;

        // Assert
        input_base::received_nothing();
        dependency::received(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn output_base_Ok() {
        // Arrange
        output_base::setup().call_base();

        // Act
        let actual_result = output_base().await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        output_base::received_nothing();
        dependency::received(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn input_output_base_Ok() {
        // Arrange
        let value = 1;
        input_output_base::setup(Arg::Any).call_base();

        // Act
        let actual_result = input_output_base(value).await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        input_output_base::received_nothing();
        dependency::received(Times::Once).no_other_calls();
    }
}
