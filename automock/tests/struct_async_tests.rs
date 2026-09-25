use automock::*;

const DEFAULT_RESULT: i32 = 123;

#[mock]
struct Struct;

#[mock(base)]
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

#[mock(base)]
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
        let mut mock = Struct::new();

        // Act
        mock.nothing().await;

        // Assert
        mock.received().nothing(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn input_Ok() {
        // Arrange
        let mut mock = Struct::new();
        let value = 1;

        // Act
        mock.input(value).await;

        // Assert
        mock.received().input(value, Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn output_Ok() {
        // Arrange
        let mut mock = Struct::new();
        let result = 2;
        mock.setup().output().returns(result);

        // Act
        let actual_result = mock.output().await;

        // Assert
        assert_eq!(result, actual_result);
        mock.received().output(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn input_output_Ok() {
        // Arrange
        let mut mock = Struct::new();
        let value = 1;
        let result = 2;
        mock.setup().input_output(Arg::Any).returns(result);

        // Act
        let actual_result = mock.input_output(value).await;

        // Assert
        assert_eq!(result, actual_result);
        mock.received()
            .input_output(value, Times::Once)
            .no_other_calls();
    }

    #[tokio::test]
    async fn static_nothing_Ok() {
        // Arrange
        // Act
        Struct::static_nothing().await;

        // Assert
        Struct::static_received()
            .static_nothing(Times::Once)
            .no_other_calls();
    }

    #[tokio::test]
    async fn static_input_Ok() {
        // Arrange
        let value = 1;

        // Act
        Struct::static_input(value).await;

        // Assert
        Struct::static_received()
            .static_input(value, Times::Once)
            .no_other_calls();
    }

    #[tokio::test]
    async fn static_output_Ok() {
        // Arrange
        let result = 2;
        Struct::static_setup().static_output().returns(result);

        // Act
        let actual_result = Struct::static_output().await;

        // Assert
        assert_eq!(result, actual_result);
        Struct::static_received()
            .static_output(Times::Once)
            .no_other_calls();
    }

    #[tokio::test]
    async fn static_input_output_Ok() {
        // Arrange
        let value = 1;
        let result = 2;
        Struct::static_setup()
            .static_input_output(Arg::Any)
            .returns(result);

        // Act
        let actual_result = Struct::static_input_output(value).await;

        // Assert
        assert_eq!(result, actual_result);
        Struct::static_received()
            .static_input_output(value, Times::Once)
            .no_other_calls();
    }

    #[tokio::test]
    async fn nothing_base_Ok() {
        // Arrange
        let mut mock = Struct::new();
        mock.setup().nothing_base().call_base();

        // Act
        mock.nothing_base().await;

        // Assert
        mock.received().nothing_base(Times::Once).no_other_calls();
        mock.received().dependency(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn input_base_Ok() {
        // Arrange
        let mut mock = Struct::new();
        let value = 1;
        mock.setup().input_base(Arg::Any).call_base();

        // Act
        mock.input_base(value).await;

        // Assert
        mock.received()
            .input_base(value, Times::Once)
            .no_other_calls();
        mock.received().dependency(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn output_base_Ok() {
        // Arrange
        let mut mock = Struct::new();
        mock.setup().output_base().call_base();

        // Act
        let actual_result = mock.output_base().await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        mock.received().output_base(Times::Once).no_other_calls();
        mock.received().dependency(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn input_output_base_Ok() {
        // Arrange
        let mut mock = Struct::new();
        let value = 1;
        mock.setup().input_output_base(Arg::Any).call_base();

        // Act
        let actual_result = mock.input_output_base(value).await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        mock.received()
            .input_output_base(value, Times::Once)
            .no_other_calls();
        mock.received().dependency(Times::Once).no_other_calls();
    }

    #[tokio::test]
    async fn static_nothing_base_Ok() {
        // Arrange
        Struct::static_setup().static_nothing_base().call_base();

        // Act
        Struct::static_nothing_base().await;

        // Assert
        Struct::static_received()
            .static_nothing_base(Times::Once)
            .static_dependency(Times::Once)
            .no_other_calls();
    }

    #[tokio::test]
    async fn static_input_base_Ok() {
        // Arrange
        let value = 1;
        Struct::static_setup()
            .static_input_base(Arg::Any)
            .call_base();

        // static_Act
        Struct::static_input_base(value).await;

        // Assert
        Struct::static_received()
            .static_input_base(value, Times::Once)
            .static_dependency(Times::Once)
            .no_other_calls();
    }

    #[tokio::test]
    async fn static_output_base_Ok() {
        // Arrange
        Struct::static_setup().static_output_base().call_base();

        // Act
        let actual_result = Struct::static_output_base().await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        Struct::static_received()
            .static_output_base(Times::Once)
            .static_dependency(Times::Once)
            .no_other_calls();
    }

    #[tokio::test]
    async fn static_input_output_base_Ok() {
        // Arrange
        let value = 1;
        Struct::static_setup()
            .static_input_output_base(Arg::Any)
            .call_base();

        // Act
        let actual_result = Struct::static_input_output_base(value).await;

        // Assert
        assert_eq!(DEFAULT_RESULT, actual_result);
        Struct::static_received()
            .static_input_output_base(value, Times::Once)
            .static_dependency(Times::Once)
            .no_other_calls();
    }
}
