#![allow(clippy::deref_addrof)]
use automock::*;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub numbers: Vec<i32>,
}

static mut BASE_MUT_REF: i32 = 10;

#[mock(base)]
trait Trait<'a, 'b> {
    fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32;

    fn return_mut_ref(&self) -> &'a mut i32;

    fn return_mut_ref_with_base(&self) -> &'a mut i32 {
        unsafe { &mut *&raw mut BASE_MUT_REF }
    }

    fn foo_sum(&mut self, Foo { mut numbers }: Foo) -> i32 {
        for number in numbers.iter_mut() {
            *number *= 2;
        }
        return numbers.into_iter().sum();
    }
}

#[allow(unused)]
#[mock(base)]
trait ISelfish {
    fn work(&self, _: &Self);
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn accept_ref_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let v = &&&&32;
        let r = &&&&8;
        trait_mock.setup().accept_ref(v).returns(r);

        // Act
        let actual_r = trait_mock.accept_ref(v);

        // Assert
        assert_eq!(r, actual_r);

        trait_mock
            .received()
            .accept_ref(v, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn return_mut_ref_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let r = &mut 5;
        trait_mock.setup().return_mut_ref().returns(r);

        // Act
        let actual_r = trait_mock.return_mut_ref();

        // Assert
        assert_eq!(r, actual_r);

        trait_mock
            .received()
            .return_mut_ref(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn return_mut_ref_with_base_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let r = &mut 5;
        trait_mock.setup().return_mut_ref_with_base().returns(r);

        // Act
        let actual_r = trait_mock.return_mut_ref_with_base();

        // Assert
        assert_eq!(r, actual_r);

        trait_mock
            .received()
            .return_mut_ref_with_base(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn return_mut_ref_with_base_CallBase_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        trait_mock.setup().return_mut_ref_with_base().call_base();

        // Act
        let actual_r = trait_mock.return_mut_ref_with_base();

        // Assert
        unsafe {
            assert_eq!(&*&raw mut BASE_MUT_REF, actual_r);
        }
        trait_mock
            .received()
            .return_mut_ref_with_base(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn foo_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let v = Foo {
            numbers: vec![5, 55],
        };
        let r = 42;
        trait_mock.setup().foo_sum(v.clone()).returns(r);

        // Act
        let actual_r = trait_mock.foo_sum(v.clone());

        // Assert
        assert_eq!(r, actual_r);

        trait_mock
            .received()
            .foo_sum(v, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn foo_CallBase_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let v = Foo {
            numbers: vec![5, 55],
        };
        trait_mock.setup().foo_sum(v.clone()).call_base();

        // Act
        let actual_r = trait_mock.foo_sum(v.clone());

        // Assert
        let expected_r = v.numbers.iter().sum::<i32>() * 2;
        assert_eq!(expected_r, actual_r);

        trait_mock
            .received()
            .foo_sum(v, Times::Once)
            .no_other_calls();
    }
}
