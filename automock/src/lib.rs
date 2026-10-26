//! Library for mocking static functions, traits and structures in Rust.
//!
//! # Usage
//! Apply `#[mock]` attribute on your function, trait, structure or `impl` block.
//!
//! ```
//! use automock::*;
//! #[mock] trait Trait {}
//!
//! # fn main() {
//! let mock = TraitMock::new();
//! # }
//! ```
//!
//! Automatically generated mock structure has two special methods to control its behavior:
//! `setup()` and `verify_calls()`. `setup()` defines expected calls and their return values.
//! `verify_calls()` checks that all expected calls happened. Unexpected calls panic immediately.
//! ```
//! # use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     fn work(&self, v: i32) -> i32;
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();    // `mock` must be mutable in order to be configured
//! mock.setup()
//!     .work(1).returns_once(10);
//!
//! // Act
//! let result = mock.work(1);
//!
//! // Assert
//! assert_eq!(result, 10);
//! mock.verify_calls();
//! # }
//! ```
//!
//! # User guide
//!
//! * [Mocking traits](#mocking-traits)
//! * [Mocking structures](#mocking-structures)
//! * [Mocking trait implementations](#mocking-trait-implementations)
//! * [Mocking static functions](#mocking-static-functions)
//! * [Mocking static associated functions](#mocking-static-associated-functions)
//! * [Arguments matching](#arguments-matching)
//! * [Comparing mocks](#comparing-mocks)
//! * [Controlling function behavior](#controlling-function-behavior)
//! * [Base implementation](#base-implementation)
//! * [Verifying calls](#verifying-calls)
//! * [Generics](#generics)
//! * [Associated constants and types](#associated-constants-and-types)
//! * [`impl Trait` types](#impl-trait-types)
//! * [Trait modifiers](#trait-modifiers)
//! * [Function modifiers](#function-modifiers)
//! * [Call order validation](#call-order-validation)
//! * [Receiver types](#receiver-types)
//! * [Cloning mocks](#cloning-mocks)
//! * [Crate features](#feature-flags)
//! * [Undefined behavior](#undefined-behavior)
//!
//! ## Mocking traits
//!
//! To mock a trait, add `#[mock]` attribute to it.
//! ```
//! use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     fn work(&self, v: i32) -> i32;
//! }
//!
//! fn use_trait(t: &dyn Trait, v: i32) -> i32 {
//!     t.work(v)
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! mock.setup()
//!     .work(1).returns_once(10)
//!     .work(2).returns_once(20);
//!
//! // Act
//! let first  = use_trait(&mock, 1);
//! let second = use_trait(&mock, 2);
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 20);
//! mock.verify_calls();
//! # }
//! ```
//!
//! ### Limitations
//!
//! There is one limitation: `automock` cannot mock traits that have super traits (other than
//! `Clone`). For example, this won't compile:
//! ```compile_fail
//! #[mock] trait Trait: SuperTrait {}
//! ```
//! But this will:
//! ```
//! # use automock::*;
//! #[mock] trait Trait: Clone {}
//! # fn main() {}
//! ```
//!
//! ## Mocking structures
//!
//! `automock` supports mocking structures, but it is primarily intended for structures that
//! behave like "stateful" functions.  
//! To mock a structure, add `#[mock]` attribute to structure definition and `#[mock]` to its
//! `impl` blocks whose functionality you want to mock.
//! ```
//! use automock::*;
//!
//! #[mock]
//! struct Struct { v: i32 }
//!
//! #[mock]
//! impl Struct {
//!     pub fn new(v: i32) -> Self { Self { v } }
//! }
//!
//! #[mock]
//! impl Struct {
//!     pub fn get(&self) -> i32 { self.v }
//! }
//!
//! # fn main() {
//! // Arrange
//! Struct::static_setup().new(Arg::Any).call_base();
//! let mut mock = Struct::new(10);
//! mock.setup().get().returns_once(20);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.verify_calls();
//! # }
//! ```
//!
//! A common pattern when mocking structures is to add a public constructor such as `new` and mock
//! its base implementation:
//!
//! ```
//! # use automock::*;
//! #[mock]
//! struct Struct;
//!
//! #[mock]
//! impl Struct {
//!     pub fn new() -> Self { Self }
//! }
//!
//! # fn main() {}
//! ```
//!
//! ### Limitations
//!
//! There are a couple of limitations for structures mocking:
//! 1. Mocked structure can not be constructed or deconstructed outside of associated functions that
//!    were mocked. This is because `automock` adds special `__mock_data` field to generated
//!    structure that it automatically fills inside mocked `impl` block.
//! 2. Structure must have either named fields or no fields at all. `struct Struct { v: i32 }` and
//!    `struct Struct;` can be mocked, but `struct Struct(i32);` can not.
//! 3. Only functions inside mocked `impl` blocks can be mocked. In the example below only `foo` can
//!    be mocked; `bar` will always use base implementation:
//!    ```
//!    # use automock::*;
//!    #[mock] struct Struct;
//!
//!    #[mock]
//!    impl Struct {
//!        fn foo(&self) -> i32 { 1 } // mockable
//!    }
//!
//!    impl Struct {
//!        fn bar(&self) -> i32 { 2 } // unmockable - will always return 2
//!    }
//!
//!    # fn main() {}
//!    ```
//! 4. A structure's `impl` block can be mocked only if the structure itself is mocked (i.e. it has
//!    `#[mock]`attribute).
//! 5. Can not mock functions with `#[cfg]` attribute. This won't compile:
//!    ```compile_fail
//!    #[mock]
//!    impl Struct {
//!        #[cfg(test)]      fn work(&self) {}
//!        #[cfg(not(test))] fn work(&self) {}
//!    }
//!    ```
//!
//! ## Mocking trait implementations
//! To mock implementations of traits on mockable structures (trait itself does not need to be
//! mockable), add `#[mock]` on `impl` block. Each mocked trait implementation
//! adds `as_TRAIT_NAME` method to mock's `setup()`. `verify_calls()` also checks calls
//! configured through these trait implementations:
//! ```
//! use automock::*;
//!
//! #[mock] struct Struct;
//! #[mock]
//! impl Struct {
//!     pub fn new() -> Self { Self }
//! }
//!
//! trait Trait {
//!     fn get(&self) -> i32;
//! }
//!
//! #[mock]
//! impl Trait for Struct {
//!     fn get(&self) -> i32 { 10 }
//! }
//!
//! # fn main() {
//! // Arrange
//! Struct::static_setup().new().call_base();
//! let mut mock = Struct::new();
//! mock.setup().as_Trait().get().returns_once(20);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.verify_calls();
//! # }
//! ```
//!
//! ### Limitations
//!
//! There are a couple of limitations for trait implementations mocking:
//! 1. Can not mock more than one implementation of same trait on a struct differing only in trait's
//!    generics. For example, this won't compile:
//!    ```compile_fail
//!    # use automock::*;
//!    #[mock]
//!    impl From<i32> for Struct {
//!        //...
//!    }
//!    #[mock]
//!    impl From<usize> for Struct {
//!        //...
//!    }
//!    ```
//!    Can mock only one of them:
//!    ```
//!    # use automock::*;
//!    # #[mock] struct Struct;
//!    #[mock]
//!    impl From<i32> for Struct {
//!    #    fn from(value: i32) -> Self { Self }
//!        //...
//!    }
//!    impl From<usize> for Struct {
//!    #    fn from(value: usize) -> Self { Self { __mock_data: Default::default() } }
//!        //...
//!    }
//!    # fn main() {}
//!    ```
//! 2. If trait has default implementations for some methods, these methods can be mocked only if
//!    they are defined inside structure's `impl` block. For example:
//!    ```
//!    # use automock::*; #[mock] struct Foo; #[mock] struct Bar;
//!    trait Trait {
//!        fn get(&self) -> i32 { 10 }
//!    }
//!
//!    // Cannot mock `Trait::get()`
//!    #[mock]
//!    impl Trait for Foo {}
//!
//!    // Can mock `Trait::get()`
//!    #[mock]
//!    impl Trait for Bar {
//!        fn get(&self) -> i32 { 10 }
//!    }
//!    # fn main() {}
//!    ```
//! 3. Limitations from [`Mocking structures`](#mocking-structures).
//!
//! ## Mocking static functions
//!
//! To mock a static function, add `#[mock]` attribute. This will generate module with the same name
//! as mocked function, which exposes standalone `setup()` and `verify_calls()` functions:
//!
//! ```
//! use automock::*;
//!
//! #[mock]
//! fn work(v: i32) -> i32 { v + 1 }
//!
//! # fn main() {
//! // Arrange
//! work::setup(1).returns_once(10)
//!      .setup(2).returns_once(20);
//!
//! // Act
//! let first  = work(1);
//! let second = work(2);
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 20);
//! work::verify_calls();
//! # }
//! ```
//!
//! ### Limitations
//!
//! There are a couple of limitations for static functions mocking:
//! 1. Configuration for standalone function mock is stored in thread-local storage to prevent race
//!    condition when running multiple tests in parallel. This may impact tests running on
//!    work-stealing async runtimes.
//! 2. Calling `setup()` of standalone function clears all its previous configurations to prevent
//!    configuration from one test leaking into next sequentially ran test. Standalone function
//!    set-up must happen in **single module-level `setup()` call** in each unit-test.  
//!    For example, this is a wrong way of configuring standalone function:
//!    ```no_run
//!    # use automock::*;
//!    #[mock] fn work(v: i32) -> i32 { v }
//!   
//!    work::setup(1).returns_once(10);
//!    work::setup(2).returns_once(20); // `work::setup(2)` will clear previous configuration
//!    ```
//!    The correct approach is to call `work::setup()` only once and then chain subsequent `.setup()`
//!    calls:
//!    ```no_run
//!    # use automock::*; #[mock] fn work(v: i32) -> i32 { v }
//!    # fn main() {
//!    work::setup(1).returns_once(10)
//!         .setup(2).returns_once(20); // `.setup(2)` does not clear previous configuration
//!    }
//!    ```
//!
//! ## Mocking static associated functions
//!
//! Static associated functions are mocked almost the same way as regular static functions, except
//! that instead of `setup()` and `verify_calls()` you must use `static_setup()` and
//! `verify_static_calls()`. Here's example of mocking trait with static function:
//! ```
//! use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     fn work(v: i32) -> i32;
//! }
//!
//! fn use_trait_impl<T: Trait>(v: i32)  -> i32 { T::work(v) }
//!
//! # fn main() {
//! // Arrange
//! TraitMock::static_setup().work(10).returns_once(20);
//!
//! // Act
//! let result = use_trait_impl::<TraitMock>(10);
//!
//! // Assert
//! assert_eq!(result, 20);
//! TraitMock::verify_static_calls();
//! # }
//! ```
//!
//! Static functions in structure implementations can also be mocked:
//! ```
//! use automock::*;
//!
//! #[mock] struct Struct;
//!
//! #[mock]
//! impl Struct {
//!     fn work(v: i32) -> i32 { v + 1 }
//! }
//!
//! # fn main() {
//! // Arrange
//! Struct::static_setup().work(10).returns_once(20);
//!
//! // Act
//! let result = Struct::work(10);
//!
//! // Assert
//! assert_eq!(result, 20);
//! Struct::verify_static_calls();
//! # }
//! ```
//!
//! ### Limitations
//!
//! Same limitations from [`Mocking static functions`](#mocking-static-functions) apply here.
//!
//! ## Arguments matching
//!
//! `setup()` functions accept [`Arg<T>`] as arguments, where `T` is type of
//! argument in source function. `Arg` provides multiple ways to match argument's value:
//!
//! 1. [`Arg::eq`] - checks that argument is equal to provided value. Uses [`PartialEq::eq`] of `T`.
//!    Can be used either manually like `mock.setup().work(Arg::eq(10))` or implicitly like
//!    `mock.setup().work(10)`.
//! 2. [`Arg::is`] - checks that argument passes provided predicate. Usage example:
//!    `mock.setup().work(Arg::is(|v: &i32| *v == 10))`. Requires specifying closure's argument type.
//! 3. [`Arg::is_mut`] - same as `Arg::is`, but receives mutable reference to argument. Can be
//!    useful for matching `FnMut` arguments.
//! 4. [`Arg::not_eq`] - checks that argument is NOT equal to provided value. Uses [`PartialEq::eq`]
//!    of `T`. Opposite of `Arg::eq`. Usage example: `mock.setup().work(Arg::not_eq(10))`.
//! 5. [`Arg::ref_eq`] - checks that argument's reference points to the same location as provided
//!    reference. Compares references returned by [`std::ops::Deref::deref`] of `T`. Usage example:
//!    ```
//!    # use automock::*;
//!    # use std::rc::Rc;
//!    #[mock]
//!    trait Trait {
//!        fn work(&self, r: Rc<i32>) -> i32;
//!    }
//!
//!    # fn main() {
//!    // Arrange
//!    let mut mock = TraitMock::new();
//!    let r1 = Rc::new(1);
//!    let r2 = r1.clone();
//!    mock.setup().work(Arg::ref_eq(r1.clone())).returns(10, 2.times());
//!
//!    // Act
//!    let first  = mock.work(r1.clone());
//!    let second = mock.work(r2);
//!
//!    // Assert
//!    assert_eq!(first,  10);
//!    assert_eq!(second, 10);
//!    mock.verify_calls();
//!    # }
//!    ```
//! 6. [`Arg::ref_not_eq`] - checks that argument's reference DOES NOT point to the same place as
//!    provided reference. Compares references returned by [`std::ops::Deref::deref`] of `T`.
//!    Opposite of `Arg::ref_eq`.
//!
//! ## Comparing mocks
//!
//! All mock objects implement [`Mockable`]. This interface provides [`Mockable::id`] method that
//! returns mock object's id - unique number that represents concrete mock object. This number is
//! unique regardless of mock object type - it can not be repeated across mocks of different types.
//!
//! ```
//! # use automock::*;
//! #[mock] trait Trait {}
//! #[mock] struct Struct;
//! #[mock] impl Struct { pub fn new() -> Self { Self } }
//!
//! # fn main() {
//! // Arrange
//! let trait_mock = TraitMock::new();
//! Struct::static_setup().new().call_base();
//! let struct_mock = Struct::new();
//!
//! // Act
//! let trait_mock_id = trait_mock.id();
//! let struct_mock_id = struct_mock.id();
//!
//! // Assert
//! assert_ne!(trait_mock_id, struct_mock_id)
//! # }
//! ```
//!
//! You can use it to compare mock objects if they don't implement `PartialEq` or do not have other
//! meaningful ways to distinguish them:
//!
//! ```
//! # use automock::*;
//! #[mock] struct Struct;
//! #[mock] impl Struct { pub fn new() -> Self { Self } }
//! #[mock] fn work(_: Struct) {}
//!
//! # fn main() {
//! // Arrange
//! Struct::static_setup().new().call_base();
//! let struct_mock = Struct::new();
//! let struct_mock_id = struct_mock.id();
//! work::setup(Arg::is(|struct_mock: &Struct| struct_mock.id() == struct_mock_id))
//!     .expect_once();
//!
//! // Act
//! work(struct_mock);
//!
//! // Assert
//! work::verify_calls();
//! # }
//! ```
//!
//! ## Controlling function behavior
//!
//! Calling `setup()` returns [`FnConfigurator`] - type that is used to tell mocked
//! function what it should do upon receiving matching call.
//!
//! ### Setting return values
//! Mocked functions that have return values can set them in multiple ways:
//! 1. [`FnConfigurator::returns`] - sets return value for given number of calls, for example
//!    `.returns(10, 2.times())`. Return value must implement [`Clone`].
//! 2. [`FnConfigurator::returns_many`] - sets multiple single-use return values in one call.
//! 3. [`FnConfigurator::returns_once`] - sets single-use return value.
//! 4. [`FnConfigurator::returns_with`] - calculates return value on the fly using function's source
//!    arguments. Receives tuple of argument references and expected call count, for example
//!    `.returns_with(|(v,)| *v + 10, 2.times())`.
//! 5. [`FnConfigurator::returns_once_with`] - like `returns_with` but returns once.
//!
//! For functions without return value:
//! * [`FnConfigurator::expect`] - expects given number of calls, for example `.expect(2.times())`.
//! * [`FnConfigurator::expect_once`] - expects one call.
//!
//! If multiple return values were specified they will be used in the same order:
//! ```
//! use automock::*;
//!
//! #[mock] fn work(v: i32) -> i32 { v + 1 }
//!
//! # fn main() {
//! // Arrange
//! work::setup(Arg::Any).returns_once(10)                         // config #1
//!      .setup(Arg::Any).returns_many([20, 30])                   // config #2
//!      .setup(Arg::Any).returns_with(|(v,)| *v + 10, 2.times()); // config #3
//!
//! // Act
//! let first  = work(1);   // uses config #1
//! let second = work(2);   // uses config #2
//! let third  = work(3);   // uses config #2
//! let fourth = work(4);   // uses config #3
//! let fifth  = work(5);   // uses config #3
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 20);
//! assert_eq!(third,  30);
//! assert_eq!(fourth, 14);
//! assert_eq!(fifth,  15);
//! # }
//! ```
//!
//! ### Setting callbacks
//! Every mocked function can have a callback that is called when it is called with matching
//! arguments. It can be configured using [`FnCallbackConfigurator::and_does`] after its return
//! value or the number of expected calls was specified. Static functions receive tuple of argument
//! values in the callback:
//! ```
//! use automock::*;
//!
//! #[mock] fn get(v: i32) -> i32 { v + 1 }
//! #[mock] fn set(v: i32) {}
//!
//! # fn main() {
//! get::setup(Arg::Any)    // Function with return value
//!     .returns_once(10)   // must set return value first
//!     .and_does(|(v,)| assert_eq!(*v, 10));
//!
//! set::setup(Arg::Any)
//!     .expect_once()
//!     .and_does(|(v,)| assert_eq!(*v, 20));
//! get(10);
//! set(20);
//! # }
//! ```
//! Associated functions receive reference to mock and tuple of argument values in the callback:
//! ```
//! use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     fn get(&self) -> i32;
//!     fn work(&self, v: i32);
//! }
//!
//! # fn main() {
//! let mut mock = TraitMock::new();
//! mock.setup()
//!     .get().returns_once(10)
//!     .work(Arg::Any)
//!     .expect_once()
//!     .and_does(|mock_ref, (v,)| {
//!         assert_eq!(mock_ref.get(), 10);
//!         assert_eq!(*v, 20);
//!     });
//! mock.work(20);
//! # }
//! ```
//!
//! ## Base implementation
//!
//! Mocked functions can use their base implementation in tests. To tell mock object to use base
//! implementation call `call_base()` on function setup:
//!
//! ```
//! use automock::*;
//!
//! #[mock] fn dependency() {}
//! #[mock]
//! fn work() {
//!     dependency();   // will be called in test
//! }
//!
//! # fn main() {
//! // Arrange
//! dependency::setup().expect_once();
//! work::setup().call_base();
//!
//! // Act
//! work();
//!
//! // Assert
//! dependency::verify_calls();
//! # }
//! ```
//!
//! Functions that have return values treat `call_base()` as return value configuration, so you can
//! not call any of `returns` functions after enabling base implementation:
//!
//! ```compile_fail
//! # use automock::*;
//! #[mock] fn work() -> i32 { 1 }
//! work::setup().call_base().returns_once(10);
//!                        // ^^^^^^^^^^^^ - error, return value is already
//!                        //                provided by base implementation
//! ```
//!
//! In trait mocks, only functions with default implementation can use `call_base()`:
//! ```
//! use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     fn dependency(&self);               // no default implementation - can not use `call_base()`
//!     fn work(&self, v: i32) -> i32 {     // has default implementation - can use `call_base()`
//!         self.dependency();
//!         return v + 1;
//!     }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! mock.setup()
//!     .work(10).call_base()
//!     .work(20).returns_once(30)
//!     .dependency().expect_once();
//!
//! // Act
//! let first  = mock.work(10);
//! let second = mock.work(20);
//!
//! // Assert
//! assert_eq!(first,  11);
//! assert_eq!(second, 30);
//! mock.verify_calls();
//! # }
//! ```
//!
//! ## Verifying calls
//!
//! You can verify exactly how a mocked function was called. To do it, just call `verify_calls()`
//! on mock:
//! ```
//! use automock::*;
//!
//! #[mock] fn work() {}
//!
//! # fn main() {
//! work::setup().expect_once();
//! work();
//! work::verify_calls();
//! # }
//! ```
//!
//! Missing call is reported when verification runs:
//!
//! ```should_panic
//! use automock::*;
//!
//! #[mock] fn work() {}
//!
//! work::setup().expect_once();
//! work::verify_calls();
//! ```
//!
//! You can verify exact values of passed arguments using [`Arg`], for more information see
//! [Arguments matching](#arguments-matching). Here's simple example that checks that mocked
//! function was called with specific arguments:
//!
//! ```
//! use automock::*;
//!
//! #[mock] fn work(_: i32, _: &str) {}
//!
//! # fn main() {
//! // Arrange
//! let number = 10;
//! let string = "quo vadis";
//! work::setup(
//!     Arg::is(|actual_number: &i32| *actual_number == number),
//!     Arg::not_eq("veridis quo"),
//! ).expect_once();
//!
//! // Act
//! work(number, string);
//!
//! // Assert
//! work::verify_calls();
//! # }
//! ```
//!
//! Unexpected argument values also panic at call site:
//! ```should_panic
//! # use automock::*;
//! #[mock]
//! trait Trait { fn set(&self, _: i32); }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! mock.setup().set(10).expect_once()
//!     .set(20).expect_once();
//!
//! // Act
//! mock.set(10);
//! mock.set(20);
//! mock.set(30);
//!
//! // Assert
//! mock.verify_calls();
//! # }
//! ```
//!
//! ## Generics
//!
//! `automock` supports generics in functions, traits and structures. They are transferred to
//! generated mocks as is. Here's a simple generics usage example with trait:
//! ```
//! use automock::*;
//!
//! #[mock]
//! trait Trait<T> { fn get(&self) -> T; }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::<i32>::new();
//! mock.setup().get().returns_once(10);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 10);
//! mock.verify_calls();
//! # }
//!
//! ```
//!
//! Here's more complex example of generics usage with structure:
//!
//! ```
//! # use std::marker::PhantomData;
//! # use std::fmt::{Debug, Display};
//! use automock::*;
//!
//! #[mock]
//! struct Struct<'a, T1: Clone + ToString, T2>
//!     where T2: Debug
//! {
//!     t1: T1,
//!     t2: &'a T2
//! }
//!
//! #[mock]
//! impl<'a, T1: Clone + ToString, T2> Struct<'a, T1, T2>
//!     where T2: Debug
//! {
//!     fn new<'b, T3: Debug + ?Sized>(t1: T1, t2: &'a T2, t3: &'b T3) -> Self
//!         where T3: Display
//!     {
//!         Self { t1, t2 }
//!     }
//!
//!     fn get_t1(&self) -> T1 {
//!         self.t1.clone()
//!     }
//! }
//!
//! # fn main() {
//! // Arrange
//! Struct::static_setup()
//!        .new(Arg::<i32>::Any, Arg::<&[i32; 3]>::Any, Arg::<&str>::Any)
//!        .call_base();
//! let mut mock = Struct::new::<str>(10i32, &[1, 2, 3], "quo vadis");
//! mock.setup().get_t1().returns_once(20);
//!
//! // Act
//! let result = mock.get_t1();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.verify_calls();
//! # }
//! ```
//!
//! Static functions use separate configurations for each generics combination:
//! ```
//! use automock::*;
//!
//! #[mock] fn get<T1, T2: Default>(t1: T1) -> T2 { T2::default() }
//!
//! # fn main() {
//! // Arrange
//! get::setup::<i32, &'static str>(10)
//!     .returns_once("quo vadis");
//! get::setup::<[u8; 3], &'static str>([1, 2, 3])
//!     .returns_once("veridis quo")
//!     .setup([10, 10, 10])        // generic types are inferred here by `get::setup` function,
//!     .returns_once("third call"); // no need to specify them again
//!
//! // Act
//! let first:  &str = get(10i32);
//! let second: &str = get([1u8, 2, 3]);
//! let third:  &str = get([10u8, 10, 10]);
//!
//! // Assert
//! assert_eq!(first,  "quo vadis");
//! assert_eq!(second, "veridis quo");
//! assert_eq!(third,  "third call");
//! get::verify_calls::<i32, &str>();
//! get::verify_calls::<[u8; 3], &str>();
//! # }
//! ```
//!
//! ## Associated constants and types
//!
//! When mocking a trait with associated constants and types, the mock type exposes them as generic
//! parameters by appending them to the source generics list in the same order in which they were
//! defined in trait:
//! ```
//! # use std::fmt::Debug;
//! use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     type Item: Debug + Default;
//!     const NUMBER: usize;
//!
//!     fn get_item(&self) -> Self::Item;
//!     fn get_number(&self) -> usize {
//!         Self::NUMBER
//!     }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::<i32, 20>::new();
//!                          // |    \
//!                          // |     Trait::NUMBER
//!                          // Trait::Item
//! mock.setup()
//!     .get_item().returns_once(10)
//!     .get_number().call_base();
//!
//! // Act
//! let item = mock.get_item();
//! let number = mock.get_number();
//!
//! // Assert
//! assert_eq!(item, 10);
//! assert_eq!(number, 20);
//! mock.verify_calls();
//! # }
//! ```
//!
//! ### Limitations
//!
//! There is one limitation - when mocking implementation of trait with associated types for struct
//! all types must be referenced explicitly like `<Self as Trait>::AssociatedType`. Using
//! `Self::AssociatedType` will lead to compilation error:
//! ```compile_fail
//! # use automock::*;
//! trait Trait {
//!     type Item;
//!     fn get_item(&self) -> Self::Item;
//! }
//!
//! #[mock]
//! struct Struct;
//! #[mock]
//! impl Trait for Struct {
//!     type Item = i32;
//!     fn get_item(&self) -> Self::Item {
//!         10             // ^^^^^^^^^^
//!     }                  // implicit associated item breaks compilation
//! }
//!
//! # fn main() {}
//! ```
//! Here's the fix:
//! ```
//! # use automock::*;
//! # trait Trait {
//! #     type Item;
//! #     fn get_item(&self) -> Self::Item;
//! # }
//! # #[mock]
//! # struct Struct;
//! #[mock]
//! impl Trait for Struct {
//!     type Item = i32;
//!     fn get_item(&self) -> <Self as Trait>::Item {
//!         10              // ^^^^^^^^^^^^^
//!                         // explicit associated item
//!     }
//! }
//!
//! # fn main() {}
//! ```
//!
//! ## `impl Trait` types
//!
//! Functions that accept or return `impl Trait` can also be mocked. Their mocking is different only
//! in one regard: if mocked function returns `impl Trait`, then its return type is replaced with
//! `Box<dyn Trait>`. Other than that such functions are mocked as usual:
//! ```
//! # use std::fmt::Debug;
//! use automock::*;
//!
//! #[mock]
//! fn work(v: impl Debug) -> impl ToString { 1 }
//!
//! # fn main() {
//! // Arrange
//! work::setup(Arg::Any).returns_once(Box::new(20));
//!
//! // Act
//! let result = work("whatever");
//!
//! // Assert
//! assert_eq!(result.to_string(), "20");
//! # }
//! ```
//!
//! ### Limitations
//!
//! There are a couple of limitations:
//! 1. `Trait` in `arg: impl Trait` must be dyn-compatible.
//! 2. Because `Trait` must be dyn-compatible, and [`PartialEq`] is not dyn-compatible, the only way
//!    to compare arguments is to use [`Arg::is`]. Alternatively, you can use [`Arg::Any`] if you
//!    don't need to check for concrete argument value.
//! 3. Can not use multiple trait bounds like `impl Foo + Bar`, in that case you will need to
//!    rewrite your function replacing this argument's type with `T: Foo + Bar`.
//! 4. A mocked function can return `impl Trait` only if `Trait` is implemented for `Box<dyn Trait>`
//!    because under the hood `automock` replaces return value with `Box<dyn Trait>` in generated
//!    implementations.
//! 5. Nested `impl Trait` are not supported. For example, this cannot be mocked:
//!    `fn f(_: impl IntoIterator<Item = impl Debug>) {}`
//!
//! ## Trait modifiers
//!
//! Only `unsafe` trait modifier is supported. It can be used both in trait definition and in its
//! implementation:
//! ```
//! use automock::*;
//! # #[mock] struct Struct;
//!
//! #[mock] unsafe trait Trait {}
//! #[mock] unsafe impl Trait for Struct {}
//! # fn main() {}
//! ```
//!
//! ## Function modifiers
//!
//! The following modifiers are supported: `async`, `unsafe` and `extern`, both in standalone and
//! associated functions:
//!
//! ```
//! use automock::*;
//!
//! #[mock] async fn async_dep() {}
//! #[mock] unsafe fn unsafe_dep() {}
//! #[mock] extern "C" fn extern_dep() {}
//!
//! #[mock]
//! # #[allow(improper_ctypes_definitions)]
//! async unsafe extern "C" fn modifiers() {
//!     async_dep().await;
//!     unsafe_dep();
//!     extern_dep();
//! }
//!
//! #[mock]
//! trait Trait {
//! # #[allow(improper_ctypes_definitions)]
//!     async unsafe extern "C" fn modifiers(&self) {
//!         async_dep().await;
//!         unsafe { unsafe_dep(); }
//!         extern_dep();
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! ## Call order validation
//!
//! To verify that calls were received in the specific, define ordered expectations inside
//! [`in_sequence`] before running code under test. Then call `.verify()` on returned `sequence`:
//! ```
//! use automock::*;
//!
//! #[mock] fn set(_: i32) {}
//!
//! # fn main() {
//! let sequence = in_sequence(|| {
//!     set::setup(1).expect_once()
//!         .setup(2).expect_once()
//!         .setup(3).expect_once();
//! });
//!
//! // Act
//! set(1);
//! set(2);
//! set(3);
//!
//! // Assert
//! set::verify_calls();
//! sequence.verify();
//! # }
//! ```
//!
//! If call order was violated `sequence.verify()` will panic. For example, this panics:
//! ```should_panic
//! # use automock::*; #[mock] fn set(_: i32) {}
//! # fn main() {
//! let sequence = in_sequence(|| {
//!     set::setup(1).expect_once()
//!         .setup(2).expect_once()
//!         .setup(3).expect_once();
//! });
//!
//! // Act
//! set(2);
//! set(1);
//! set(3);
//!
//! // Assert
//! set::verify_calls();
//! sequence.verify(); // panic: expected order 1-2-3, actually got 2-1-3
//! # }
//! ```
//!
//! Call order is verified for all mocked functions relative to each other, regardless if they come
//! from different functions or even mock objects:
//! ```
//! use automock::*;
//!
//! #[mock] fn first() {}
//!
//! #[mock]
//! trait Trait {
//!     fn second(&self);
//! }
//!
//! #[mock] struct Struct;
//! #[mock]
//! impl Struct {
//!     pub fn new() -> Self { Self }
//!     pub fn third(&self) {}
//! }
//!
//! # fn main() {
//! // Arrange
//! Struct::static_setup().new().call_base();
//! let mut trait_mock = TraitMock::new();
//! let mut struct_mock = Struct::new();
//! let sequence = in_sequence(|| {
//!     first::setup().expect_once();
//!     trait_mock.setup().second().expect_once();
//!     struct_mock.setup().third().expect_once();
//! });
//!
//! // Act
//! first();
//! trait_mock.second();
//! struct_mock.third();
//!
//! // Assert
//! first::verify_calls();
//! trait_mock.verify_calls();
//! struct_mock.verify_calls();
//! sequence.verify();
//! # }
//! ```
//!
//! ## Receiver types
//!
//! Associated functions can have any kind of receiver type: `&Self`, `Self`, `Rc<Self>` or even
//! nested types like `Box<Rc<Arc<&Self>>>`:
//! ```
//! # use std::rc::Rc;
//! # use std::sync::Arc;
//! use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     fn fn_self(self);
//!     fn fn_ref_self(&self);
//!     fn fn_rc_self(self: Rc<Self>);
//!     fn fn_nested_self(self: Box<Rc<Arc<&Self>>>);
//! }
//!
//! # fn main() {}
//! ```
//!
//! To mock them you don't need to put mock in the same container that is used in source function's
//! signature:
//! ```
//! use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     fn boxed(self: Box<Self>) -> i32;
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! mock.setup().boxed().returns_once(10); // setup() and verify_calls() are always called on "&mut self"
//!
//! // Act
//! let result = Box::new(mock.clone()).boxed();    // another feature - mocks cloning, more
//!                                                 // information in "Cloning mocks" section
//!
//! // Assert
//! assert_eq!(result, 10);
//! mock.verify_calls();
//! # }
//! ```
//!
//! ## Cloning mocks
//!
//! Mock can be cloned either if:
//! 1. it is mock of a trait,
//! 2. it is mock of a struct that has `#[derive(Clone)]` attribute (manually implementing [`Clone`]
//!    would not work).
//!
//! Cloned mocks share the same configuration (it is stored behind reference-counted pointer
//! internally). This lets you share mocks between parts of your code. This can be useful, for
//! example, if you want to verify that mock received some consuming function:
//! ```
//! use automock::*;
//!
//! #[mock]
//! trait Trait {
//!     fn consume(self);
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! let mut mock_for_verification = mock.clone();
//! mock.setup().consume().expect_once();
//!
//! // Act
//! mock.consume();
//!
//! // Assert
//! mock_for_verification.verify_calls();
//! # }
//! ```
//!
//! ## Feature flags
//!
//! ### `debug_naming`
//!
//! Enables better generic argument values display in error messages.
//!
//! Without this feature arguments that have generic type will be printed as just "?" in errors,
//! even if they implement [`std::fmt::Debug`]. With this feature such arguments will be printed as
//! their debug strings if they implement `Debug`. For example, without this feature in function
//! `fn work<T>(t: T)` the argument `t` will be printed as "?" in all errors.
//!
//! This feature requires nightly version of compiler because it uses Rust's
//! [`specialization`](https://rust-lang.github.io/rfcs/1210-impl-specialization.html) feature.
//!
//! # How it works
//!
//! Easiest way to mock some function is to create two separate versions of it - one for `release`
//! build and one for `dev`/`debug_assertions` build that tracks calls:
//! ```no_run
//! #[cfg(not(test))] fn f() {}
//!
//! #[cfg(test)] fn f() { F_CALLS_COUNT += 1; }
//! #[cfg(test)] static mut F_CALLS_COUNT: usize = 0;
//!
//! fn payload() { f() }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!     #[test]
//!     fn payload_test() {
//!         // Act
//!         payload();
//!
//!         // Assert
//!         assert_eq!(F_CALLS_COUNT, 1);
//!     }
//! }
//! ```
//! This is basically what `automock` does - it automatically creates infrastructure for mocking
//! that provides ways to check `F_CALLS_COUNT` and other function parameters.
pub use automock_proc_macro::mock;

#[doc(hidden)]
pub use automock_core::args::*;
pub use automock_core::in_sequence;
pub use automock_core::infrastructure::{FnCallbackConfigurator, FnConfigurator, Mockable};
pub use automock_core::times::*;
pub use automock_core::transmute_lifetime;

#[doc(hidden)]
pub mod for_generated {
    pub use automock_core::args::*;
    pub use automock_core::fn_parameters::*;
    pub use automock_core::infrastructure::*;
    pub use automock_core::times::*;
    pub use automock_core::*;
}
