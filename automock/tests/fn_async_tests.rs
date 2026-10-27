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

#[mock]
async fn nothing_base() {
    dependency().await
}

#[mock]
async fn input_base(_: i32) {
    dependency().await
}

#[mock]
async fn output_base() -> i32 {
    dependency().await;
    DEFAULT_RESULT
}

#[mock]
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
        nothing::setup().expect_once();

        // Act
        nothing().await;

        // Assert
        nothing::verify_calls();
    }

    #[tokio::test]
    async fn input_Ok() {
        // Arrange
        let value = 1;
        input::setup(value).expect_once();

        // Act
        input(value).await;

        // Assert
        input::verify_calls();
    }

    #[tokio::test]
    async fn output_Ok() {
        // Arrange
        let result = 2;
        output::setup().returns_once(result);

        // Act
        let actual_result = output().await;

        // Assert
        assert_eq!(result, actual_result);
        output::verify_calls();
    }

    #[tokio::test]
    async fn input_output_Ok() {
        // Arrange
        let value = 1;
        let result = 2;
        input_output::setup(value).returns_once(result);

        // Act
        let actual_result = input_output(value).await;

        // Assert
        assert_eq!(result, actual_result);
        input_output::verify_calls();
    }

    #[tokio::test]
    async fn nothing_base_Ok() {
        // Arrange
        nothing_base::setup().call_base();
        dependency::setup().expect_once();

        // Act
        nothing_base().await;

        // Assert
        nothing_base::verify_calls();
        dependency::verify_calls();
    }

    #[tokio::test]
    async fn input_base_Ok() {
        // Arrange
        let value = 1;
        input_base::setup(value).call_base();
        dependency::setup().expect_once();

        // Act
        input_base(value).await;

        // Assert
        input_base::verify_calls();
        dependency::verify_calls();
    }

    #[tokio::test]
    async fn output_base_Ok() {
        // Arrange
        output_base::setup().call_base();
        dependency::setup().expect_once();

        // Act
        let actual_result = output_base().await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        output_base::verify_calls();
        dependency::verify_calls();
    }

    #[tokio::test]
    async fn input_output_base_Ok() {
        // Arrange
        let value = 1;
        input_output_base::setup(value).call_base();
        dependency::setup().expect_once();

        // Act
        let actual_result = input_output_base(value).await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        input_output_base::verify_calls();
        dependency::verify_calls();
    }
}
