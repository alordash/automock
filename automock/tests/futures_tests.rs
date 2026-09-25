use automock::*;

const DEFAULT: i32 = 10;
const MOCKED: i32 = 20;

#[mock]
struct Struct;

#[mock(base)]
impl Struct {
    pub fn new() -> Self {
        Self
    }
}

#[mock(base)]
fn work_full() -> impl core::future::Future<Output = i32> {
    async move { DEFAULT }
}

#[mock(base)]
trait TraitFull {
    fn work_full(&self) -> impl core::future::Future<Output = i32> {
        async move { DEFAULT }
    }

    fn static_work_full() -> impl core::future::Future<Output = i32> {
        async move { DEFAULT }
    }
}
#[mock(base)]
impl Struct {
    pub fn work_full(&self) -> impl core::future::Future<Output = i32> {
        async move { DEFAULT }
    }

    pub fn static_work_full() -> impl core::future::Future<Output = i32> {
        async move { DEFAULT }
    }
}

#[mock(base)]
impl TraitFull for Struct {
    fn work_full(&self) -> impl core::future::Future<Output = i32> {
        async move { DEFAULT }
    }

    fn static_work_full() -> impl core::future::Future<Output = i32> {
        async move { DEFAULT }
    }
}

use part::*;
mod part {
    use super::*;
    use core::future;

    #[mock(base)]
    pub fn work_part() -> impl future::Future<Output = i32> {
        async move { DEFAULT }
    }

    #[mock(base)]
    pub trait TraitPart {
        fn work_part(&self) -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }

        fn static_work_part() -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }
    }

    #[mock(base)]
    impl Struct {
        pub fn work_part(&self) -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }

        pub fn static_work_part() -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }
    }

    #[mock(base)]
    impl TraitPart for Struct {
        fn work_part(&self) -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }

        fn static_work_part() -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }
    }
}

use name::*;
mod name {
    use super::*;
    use core::future::Future;
    use std::future;

    #[mock(base)]
    pub fn work_name() -> impl Future<Output = i32> {
        async move { DEFAULT }
    }

    #[mock(base)]
    pub trait TraitName {
        fn work_name(&self) -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }

        fn static_work_name() -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }
    }

    #[mock(base)]
    impl Struct {
        pub fn work_name(&self) -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }

        pub fn static_work_name() -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }
    }

    #[mock(base)]
    impl TraitName for Struct {
        fn work_name(&self) -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }

        fn static_work_name() -> impl future::Future<Output = i32> {
            async move { DEFAULT }
        }
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    mod r#fn {
        use super::*;

        #[tokio::test]
        async fn work_full_Ok() {
            // Arrange
            work_full::setup().returns(Box::pin(async move { MOCKED }));

            // Act
            let result = work_full().await;

            // Assert
            assert_eq!(MOCKED, result);
            work_full::received(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_full_CallBase_Ok() {
            // Arrange
            work_full::setup().call_base();

            // Act
            let result = work_full().await;

            // Assert
            assert_eq!(DEFAULT, result);
            work_full::received(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_part_Ok() {
            // Arrange
            work_part::setup().returns(Box::pin(async move { MOCKED }));

            // Act
            let result = work_part().await;

            // Assert
            assert_eq!(MOCKED, result);
            work_part::received(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_part_CallBase_Ok() {
            // Arrange
            work_part::setup().call_base();

            // Act
            let result = work_part().await;

            // Assert
            assert_eq!(DEFAULT, result);
            work_part::received(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_name_Ok() {
            // Arrange
            work_name::setup().returns(Box::pin(async move { MOCKED }));

            // Act
            let result = work_name().await;

            // Assert
            assert_eq!(MOCKED, result);
            work_name::received(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_name_CallBase_Ok() {
            // Arrange
            work_name::setup().call_base();

            // Act
            let result = work_name().await;

            // Assert
            assert_eq!(DEFAULT, result);
            work_name::received(1.time()).no_other_calls();
        }
    }

    mod r#trait {
        use super::*;

        #[tokio::test]
        async fn work_full_Ok() {
            // Arrange
            let mut mock = TraitFullMock::new();
            mock.setup()
                .work_full()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = mock.work_full().await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received().work_full(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_full_CallBase_Ok() {
            // Arrange
            let mut mock = TraitFullMock::new();
            mock.setup().work_full().call_base();

            // Act
            let result = mock.work_full().await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received().work_full(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn static_work_full_Ok() {
            // Arrange
            TraitFullMock::static_setup()
                .static_work_full()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = TraitFullMock::static_work_full().await;

            // Assert
            assert_eq!(MOCKED, result);
            TraitFullMock::static_received()
                .static_work_full(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_full_CallBase_Ok() {
            // Arrange
            TraitFullMock::static_setup().static_work_full().call_base();

            // Act
            let result = TraitFullMock::static_work_full().await;

            // Assert
            assert_eq!(DEFAULT, result);
            TraitFullMock::static_received()
                .static_work_full(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_part_Ok() {
            // Arrange
            let mut mock = TraitPartMock::new();
            mock.setup()
                .work_part()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = mock.work_part().await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received().work_part(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_part_CallBase_Ok() {
            // Arrange
            let mut mock = TraitPartMock::new();
            mock.setup().work_part().call_base();

            // Act
            let result = mock.work_part().await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received().work_part(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn static_work_part_Ok() {
            // Arrange
            TraitPartMock::static_setup()
                .static_work_part()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = TraitPartMock::static_work_part().await;

            // Assert
            assert_eq!(MOCKED, result);
            TraitPartMock::static_received()
                .static_work_part(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_part_CallBase_Ok() {
            // Arrange
            TraitPartMock::static_setup().static_work_part().call_base();

            // Act
            let result = TraitPartMock::static_work_part().await;

            // Assert
            assert_eq!(DEFAULT, result);
            TraitPartMock::static_received()
                .static_work_part(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_name_Ok() {
            // Arrange
            let mut mock = TraitNameMock::new();
            mock.setup()
                .work_name()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = mock.work_name().await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received().work_name(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_name_CallBase_Ok() {
            // Arrange
            let mut mock = TraitNameMock::new();
            mock.setup().work_name().call_base();

            // Act
            let result = mock.work_name().await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received().work_name(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn static_work_name_Ok() {
            // Arrange
            TraitNameMock::static_setup()
                .static_work_name()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = TraitNameMock::static_work_name().await;

            // Assert
            assert_eq!(MOCKED, result);
            TraitNameMock::static_received()
                .static_work_name(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_name_CallBase_Ok() {
            // Arrange
            TraitNameMock::static_setup().static_work_name().call_base();

            // Act
            let result = TraitNameMock::static_work_name().await;

            // Assert
            assert_eq!(DEFAULT, result);
            TraitNameMock::static_received()
                .static_work_name(1.time())
                .no_other_calls();
        }
    }

    mod r#struct {
        use super::*;

        #[tokio::test]
        async fn work_full_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup()
                .work_full()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = mock.work_full().await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received().work_full(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_full_CallBase_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup().work_full().call_base();

            // Act
            let result = mock.work_full().await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received().work_full(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn static_work_full_Ok() {
            // Arrange
            Struct::static_setup()
                .static_work_full()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = Struct::static_work_full().await;

            // Assert
            assert_eq!(MOCKED, result);
            Struct::static_received()
                .static_work_full(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_full_CallBase_Ok() {
            // Arrange
            Struct::static_setup().static_work_full().call_base();

            // Act
            let result = Struct::static_work_full().await;

            // Assert
            assert_eq!(DEFAULT, result);
            Struct::static_received()
                .static_work_full(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_part_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup()
                .work_part()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = mock.work_part().await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received().work_part(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_part_CallBase_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup().work_part().call_base();

            // Act
            let result = mock.work_part().await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received().work_part(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn static_work_part_Ok() {
            // Arrange
            Struct::static_setup()
                .static_work_part()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = Struct::static_work_part().await;

            // Assert
            assert_eq!(MOCKED, result);
            Struct::static_received()
                .static_work_part(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_part_CallBase_Ok() {
            // Arrange
            Struct::static_setup().static_work_part().call_base();

            // Act
            let result = Struct::static_work_part().await;

            // Assert
            assert_eq!(DEFAULT, result);
            Struct::static_received()
                .static_work_part(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_name_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup()
                .work_name()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = mock.work_name().await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received().work_name(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn work_name_CallBase_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup().work_name().call_base();

            // Act
            let result = mock.work_name().await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received().work_name(1.time()).no_other_calls();
        }

        #[tokio::test]
        async fn static_work_name_Ok() {
            // Arrange
            Struct::static_setup()
                .static_work_name()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = Struct::static_work_name().await;

            // Assert
            assert_eq!(MOCKED, result);
            Struct::static_received()
                .static_work_name(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_name_CallBase_Ok() {
            // Arrange
            Struct::static_setup().static_work_name().call_base();

            // Act
            let result = Struct::static_work_name().await;

            // Assert
            assert_eq!(DEFAULT, result);
            Struct::static_received()
                .static_work_name(1.time())
                .no_other_calls();
        }
    }

    mod struct_as_trait {
        use super::*;

        #[tokio::test]
        async fn work_full_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup()
                .as_TraitFull()
                .work_full()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = TraitFull::work_full(&mock).await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received()
                .as_TraitFull()
                .work_full(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_full_CallBase_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup().as_TraitFull().work_full().call_base();

            // Act
            let result = TraitFull::work_full(&mock).await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received()
                .as_TraitFull()
                .work_full(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_full_Ok() {
            // Arrange
            Struct::static_setup()
                .as_TraitFull()
                .static_work_full()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = <Struct as TraitFull>::static_work_full().await;

            // Assert
            assert_eq!(MOCKED, result);
            Struct::static_received()
                .as_TraitFull()
                .static_work_full(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_full_CallBase_Ok() {
            // Arrange
            Struct::static_setup()
                .as_TraitFull()
                .static_work_full()
                .call_base();

            // Act
            let result = <Struct as TraitFull>::static_work_full().await;

            // Assert
            assert_eq!(DEFAULT, result);
            Struct::static_received()
                .as_TraitFull()
                .static_work_full(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_part_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup()
                .as_TraitPart()
                .work_part()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = TraitPart::work_part(&mock).await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received()
                .as_TraitPart()
                .work_part(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_part_CallBase_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup().as_TraitPart().work_part().call_base();

            // Act
            let result = TraitPart::work_part(&mock).await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received()
                .as_TraitPart()
                .work_part(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_part_Ok() {
            // Arrange
            Struct::static_setup()
                .as_TraitPart()
                .static_work_part()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = <Struct as TraitPart>::static_work_part().await;

            // Assert
            assert_eq!(MOCKED, result);
            Struct::static_received()
                .as_TraitPart()
                .static_work_part(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_part_CallBase_Ok() {
            // Arrange
            Struct::static_setup()
                .as_TraitPart()
                .static_work_part()
                .call_base();

            // Act
            let result = <Struct as TraitPart>::static_work_part().await;

            // Assert
            assert_eq!(DEFAULT, result);
            Struct::static_received()
                .as_TraitPart()
                .static_work_part(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_name_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup()
                .as_TraitName()
                .work_name()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = TraitName::work_name(&mock).await;

            // Assert
            assert_eq!(MOCKED, result);
            mock.received()
                .as_TraitName()
                .work_name(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn work_name_CallBase_Ok() {
            // Arrange
            let mut mock = Struct::new();
            mock.setup().as_TraitName().work_name().call_base();

            // Act
            let result = TraitName::work_name(&mock).await;

            // Assert
            assert_eq!(DEFAULT, result);
            mock.received()
                .as_TraitName()
                .work_name(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_name_Ok() {
            // Arrange
            Struct::static_setup()
                .as_TraitName()
                .static_work_name()
                .returns(Box::pin(async move { MOCKED }));

            // Act
            let result = <Struct as TraitName>::static_work_name().await;

            // Assert
            assert_eq!(MOCKED, result);
            Struct::static_received()
                .as_TraitName()
                .static_work_name(1.time())
                .no_other_calls();
        }

        #[tokio::test]
        async fn static_work_name_CallBase_Ok() {
            // Arrange
            Struct::static_setup()
                .as_TraitName()
                .static_work_name()
                .call_base();

            // Act
            let result = <Struct as TraitName>::static_work_name().await;

            // Assert
            assert_eq!(DEFAULT, result);
            Struct::static_received()
                .as_TraitName()
                .static_work_name(1.time())
                .no_other_calls();
        }
    }
}
