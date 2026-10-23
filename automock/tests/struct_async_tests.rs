use automock::*;

const DEFAULT_RESULT: i32 = 123;

#[mock]
struct Struct;

#[mock]
impl Struct {
    pub fn new() -> Self {
        Self
    }

    async fn dependency(&self) {}

    async fn static_dependency() {}
}

#[mock]
impl Struct {
    async fn nothing(&self) {}

    async fn input(&self, _: i32) {}

    async fn output(&self) -> i32 {
        unreachable!("base is not mocked")
    }

    async fn input_output(&self, _: i32) -> i32 {
        unreachable!("base is not mocked")
    }

    async fn static_nothing() {}

    async fn static_input(_: i32) {}

    async fn static_output() -> i32 {
        unreachable!("base is not mocked")
    }

    async fn static_input_output(_: i32) -> i32 {
        unreachable!("base is not mocked")
    }
}

#[mock]
impl Struct {
    async fn nothing_base(&self) {
        self.dependency().await
    }

    async fn input_base(&self, _: i32) {
        self.dependency().await
    }

    async fn output_base(&self) -> i32 {
        self.dependency().await;
        DEFAULT_RESULT
    }

    async fn input_output_base(&self, _: i32) -> i32 {
        self.dependency().await;
        DEFAULT_RESULT
    }

    async fn static_nothing_base() {
        Self::static_dependency().await
    }

    async fn static_input_base(_: i32) {
        Self::static_dependency().await
    }

    async fn static_output_base() -> i32 {
        Self::static_dependency().await;
        DEFAULT_RESULT
    }

    async fn static_input_output_base(_: i32) -> i32 {
        Self::static_dependency().await;
        DEFAULT_RESULT
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    async fn nothing_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        mock.setup().nothing().expect_once();

        // Act
        mock.nothing().await;

        // Assert
        mock.verify_calls();
    }

    #[tokio::test]
    async fn input_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let value = 1;
        mock.setup().input(value).expect_once();

        // Act
        mock.input(value).await;

        // Assert
        mock.verify_calls();
    }

    #[tokio::test]
    async fn output_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let result = 2;
        mock.setup().output().returns_once(result);

        // Act
        let actual_result = mock.output().await;

        // Assert
        assert_eq!(result, actual_result);
        mock.verify_calls();
    }

    #[tokio::test]
    async fn input_output_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let value = 1;
        let result = 2;
        mock.setup().input_output(value).returns_once(result);

        // Act
        let actual_result = mock.input_output(value).await;

        // Assert
        assert_eq!(result, actual_result);
        mock.verify_calls();
    }

    #[tokio::test]
    async fn static_nothing_Ok() {
        // Arrange
        Struct::static_setup().static_nothing().expect_once();

        // Act
        Struct::static_nothing().await;

        // Assert
        Struct::verify_static_calls();
    }

    #[tokio::test]
    async fn static_input_Ok() {
        // Arrange
        let value = 1;
        Struct::static_setup().static_input(value).expect_once();

        // Act
        Struct::static_input(value).await;

        // Assert
        Struct::verify_static_calls();
    }

    #[tokio::test]
    async fn static_output_Ok() {
        // Arrange
        let result = 2;
        Struct::static_setup().static_output().returns_once(result);

        // Act
        let actual_result = Struct::static_output().await;

        // Assert
        assert_eq!(result, actual_result);
        Struct::verify_static_calls();
    }

    #[tokio::test]
    async fn static_input_output_Ok() {
        // Arrange
        let value = 1;
        let result = 2;
        Struct::static_setup()
            .static_input_output(value)
            .returns_once(result);

        // Act
        let actual_result = Struct::static_input_output(value).await;

        // Assert
        assert_eq!(result, actual_result);
        Struct::verify_static_calls();
    }

    #[tokio::test]
    async fn nothing_base_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        mock.setup().nothing_base().call_base();
        mock.setup().dependency().expect_once();

        // Act
        mock.nothing_base().await;

        // Assert
        mock.verify_calls();
    }

    #[tokio::test]
    async fn input_base_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let value = 1;
        mock.setup().input_base(value).call_base();
        mock.setup().dependency().expect_once();

        // Act
        mock.input_base(value).await;

        // Assert
        mock.verify_calls();
    }

    #[tokio::test]
    async fn output_base_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        mock.setup().output_base().call_base();
        mock.setup().dependency().expect_once();

        // Act
        let actual_result = mock.output_base().await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        mock.verify_calls();
    }

    #[tokio::test]
    async fn input_output_base_Ok() {
        // Arrange
        Struct::static_setup().new().call_base();
        let mut mock = Struct::new();
        let value = 1;
        mock.setup().input_output_base(value).call_base();
        mock.setup().dependency().expect_once();

        // Act
        let actual_result = mock.input_output_base(value).await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        mock.verify_calls();
    }

    #[tokio::test]
    async fn static_nothing_base_Ok() {
        // Arrange
        Struct::static_setup()
            .static_nothing_base()
            .call_base()
            .static_dependency()
            .expect_once();

        // Act
        Struct::static_nothing_base().await;

        // Assert
        Struct::verify_static_calls();
    }

    #[tokio::test]
    async fn static_input_base_Ok() {
        // Arrange
        let value = 1;
        Struct::static_setup()
            .static_input_base(value)
            .call_base()
            .static_dependency()
            .expect_once();

        // static_Act
        Struct::static_input_base(value).await;

        // Assert
        Struct::verify_static_calls();
    }

    #[tokio::test]
    async fn static_output_base_Ok() {
        // Arrange
        Struct::static_setup()
            .static_output_base()
            .call_base()
            .static_dependency()
            .expect_once();

        // Act
        let actual_result = Struct::static_output_base().await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        Struct::verify_static_calls();
    }

    #[tokio::test]
    async fn static_input_output_base_Ok() {
        // Arrange
        let value = 1;
        Struct::static_setup()
            .static_input_output_base(value)
            .call_base()
            .static_dependency()
            .expect_once();

        // Act
        let actual_result = Struct::static_input_output_base(value).await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        Struct::verify_static_calls();
    }
}
