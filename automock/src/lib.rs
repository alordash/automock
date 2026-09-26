//! Library for mocking static functions, traits and structures in Rust.
//!
//! # Usage
//! Apply `#[mock]` attribute on your function, trait, structure or `impl` block. You can also
//! use `#[mock(base)]` if you want the ability to use base implementation in your tests (refer to
//! [`Base implementation`](#base-implementation) for more information).
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
//! `setup()` and `received()`. `setup()` allows you to configure how mock object behaves when
//! its methods are called. `received()` is used to check how mock object was used.
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
//!     .work(1).returns(10)    // when called as `work(1)` it will return 10
//!     .work(2).returns(20);   // when called as `work(2)` it will return 20
//!
//! // Act
//! let result = mock.work(1);
//!
//! // Assert
//! assert_eq!(result, 10);
//! mock.received()
//!     .work(1, 1.time())      // verify that `work(1)` was called once
//!     .no_other_calls();      // verify that no other calls were performed
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
//! * [Cloning mocks](#mocks-cloning)
//! * [Crate settings](#crate-settings)
//! * [Crate features](#crate-features)
//! * [Undefined behavior](#undefined-behavior)
//!
//! ## Mocking traits
//!
//! To mock a trait, add `#[mock]` or `#[mock(base)]` attribute to it.
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
//!     .work(1).returns(10)
//!     .work(2).returns(20);
//!
//! // Act
//! let first  = use_trait(&mock, 1);
//! let second = use_trait(&mock, 2);
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 20);
//! mock.received()
//!     .work(1, 1.time())
//!     .work(2, 1.time());
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
//! To mock a structure, add `#[mock]` attribute to structure definition and `#[mock]` or
//! `#[mock(base)]` to its `impl` blocks whose functionality you want to mock.
//! ```
//! use automock::*;
//!
//! #[mock]
//! struct Structure { v: i32 }
//!
//! #[mock(base)]
//! impl Structure {
//!     pub fn new(v: i32) -> Self { Self { v } }
//! }
//!
//! #[mock]
//! impl Structure {
//!     pub fn get(&self) -> i32 { self.v }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = Structure::new(10);
//! mock.setup().get().returns(20);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.received().get(1.time());
//! # }
//! ```
//!
//! A common pattern when mocking structures is to add a public constructor such as `new` and
//! mock its base implementation:
//!
//! ```
//! # use automock::*;
//! #[mock]
//! struct Struct;
//!
//! #[mock(base)]
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
//!    were mocked. This is because `automock` adds special `__am_data` field to generated
//!    structure that it automatically fills inside mocked `impl` block.
//! 2. Structure must have either named fields or no fields at all. `struct Struct { v: i32 }` and
//!    `struct Struct;` can be mocked, but `struct Struct(i32);` can not.
//! 3. Only functions inside mocked `impl` blocks can be mocked. In the example below only `foo` can
//!    be mocked; `bar` will always use base implementation:
//! ```
//! # use automock::*;
//! #[mock] struct Structure;
//!
//! #[mock]
//! impl Structure {
//!     fn foo(&self) -> i32 { 1 } // mockable
//! }
//!
//! impl Structure {
//!     fn bar(&self) -> i32 { 2 } // unmockable - will always return 2
//! }
//!
//! # fn main() {}
//! ```
//! 4. A structure's `impl` block can be mocked only if the structure itself is mocked (i.e. it has
//!    `#[mock]`attribute).
//! 5. Can not mock functions with `#[cfg]` attribute. This won't compile:
//! ```compile_fail
//! #[mock]
//! impl Structure {
//!     #[cfg(test)]      fn work(&self) {}
//!     #[cfg(not(test))] fn work(&self) {}
//! }
//! ```
//!
//! ## Mocking trait implementations
//! To mock implementations of traits on mockable structures (trait itself does not need to be
//! mockable), add `#[mock]` or `#[mock(base)]` on `impl` block. Each mocked trait implementation
//! adds `as_TRAIT_NAME` method both to mock's `setup` and `received` functions:
//! ```
//! use automock::*;
//!
//! #[mock] struct Structure;
//! #[mock(base)]
//! impl Structure {
//!     pub fn new() -> Self { Self }
//! }
//!
//! trait Trait {
//!     fn get(&self) -> i32;
//! }
//!
//! #[mock]
//! impl Trait for Structure {
//!     fn get(&self) -> i32 { 10 }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = Structure::new();
//! mock.setup().as_Trait().get().returns(20);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.received().as_Trait().get(1.time());
//! # }
//! ```
//!
//! ### Limitations
//!
//! There are a couple of limitations for trait implementations mocking:
//! 1. Can not mock more than one implementation of same trait on a struct differing only in trait's
//!    generics. For example, this won't compile:
//! ```compile_fail
//! # use automock::*;
//! #[mock]
//! impl From<i32> for Struct {
//!     //...
//! }
//! #[mock]
//! impl From<usize> for Struct {
//!     //...
//! }
//! ```
//! Can mock only one of them:
//! ```
//! # use automock::*;
//! # #[mock] struct Struct;
//! #[mock]
//! impl From<i32> for Struct {
//! #    fn from(value: i32) -> Self { Self }
//!     //...
//! }
//! impl From<usize> for Struct {
//! #    fn from(value: usize) -> Self { Self { __am_data: Default::default() } }
//!     //...
//! }
//!
//! # fn main() {}
//! ```
//! 2. If trait has default implementations for some methods, these methods can be mocked only if
//!    they are defined inside structure's `impl` block. For example:
//! ```
//! # use automock::*; #[mock] struct Foo; #[mock] struct Bar;
//! trait Trait {
//!     fn get(&self) -> i32 { 10 }
//! }
//!
//! // Cannot mock `Trait::get()`
//! #[mock]
//! impl Trait for Foo {}
//!
//! // Can mock `Trait::get()`
//! #[mock]
//! impl Trait for Bar {
//!     fn get(&self) -> i32 { 10 }
//! }
//! # fn main() {}
//! ```
//! 3. Limitations from [`Mocking structures`](#mocking-structures).
//!
//! ## Mocking static functions
//!
//! To mock a static function, add `#[mock]` or `#[mock(base)]` attribute. This will generate module
//! with the same name as mocked function, which exposes standalone `setup()` and `received()`
//! functions:
//!
//! ```
//! use automock::*;
//!
//! #[mock]
//! fn work(v: i32) -> i32 { v + 1 }
//!
//! # fn main() {
//! // Arrange
//! work::setup(1).returns(10)
//!      .setup(2).returns(20);
//!
//! // Act
//! let first  = work(1);
//! let second = work(2);
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 20);
//! work::received(1, 1.time())
//!      .received(2, 1.time());
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
//!    // TODO - fix indents in numbered lists
//!    ```no_run
//!    # use automock::*;
//!    #[mock] fn work(v: i32) -> i32 { v }
//!   
//!    work::setup(1).returns(10);
//!    work::setup(2).returns(20); // `work::setup(2)` will clear previous configuration
//!    ```
//!    The correct approach is to call `work::setup()` only once and then chain subsequent `.setup()`
//!    calls:
//!    ```no_run
//!    # use automock::*; #[mock] fn work(v: i32) -> i32 { v }
//!    # fn main() {
//!    work::setup(1).returns(10)
//!         .setup(2).returns(20); // `.setup(2)` does not clear previous configuration
//!    }
//!    ```
//!
//! ## Mocking static associated functions
//!
//! Static associated functions are mocked almost the same way as regular static functions, except
//! that instead of `setup()` and `received()` you must use `static_setup()` and
//! `static_received()`. Here's example of mocking trait with static function:
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
//! TraitMock::static_setup().work(10).returns(20);
//!
//! // Act
//! let result = use_trait_impl::<TraitMock>(10);
//!
//! // Assert
//! assert_eq!(result, 20);
//! TraitMock::static_received().work(10, 1.time());
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
//! Struct::static_setup().work(10).returns(20);
//!
//! // Act
//! let result = Struct::work(10);
//!
//! // Assert
//! assert_eq!(result, 20);
//! Struct::static_received().work(10, 1.time());
//! # }
//! ```
//!
//! ### Limitations
//!
//! There are a couple of limitations for static associated functions mocking:
//! 1. Associated functions that use base implementation using `#[mock(base)]` use base // TODO - remove this remark?
//!    implementation by default, without any configuration. This is done to make creation of
//!    structure mocks simpler by just calling `Struct::new()` without needing to first do
//!    `Struct::static_setup().new(Arg::Any, ...).call_base()` in each test.
//! 2. Limitations from [`Mocking static functions`](#mocking-static-functions)
//!
//! ## Arguments matching
//!
//! `setup()` and `received()` functions accept [`Arg<T>`] as arguments, where `T` is type of
//! argument in source function. `Arg` provides multiple ways to match argument's value:
//!
//! 1. [`Arg::eq`] - checks that argument is equal to provided value. Uses [`PartialEq::eq`] of `T`.
//!    Can be used either manually like `mock.setup(Arg::eq(10))` or implicitly like
//!    `mock.setup(10)`.
//! 2. [`Arg::is`] - checks that argument passes provided predicate. Usage example:
//!    `mock.setup(Arg::is(|v: &i32| *v == 10))`. Requires specifying closure's argument type.
//! 3. [`Arg::not_eq`] - checks that argument is NOT equal to provided value. Uses [`PartialEq::eq`]
//!    of `T`. Opposite of `Arg::eq`. Usage example: `mock.setup(Arg::not_eq(10))`.
//! 4. [`Arg::ref_eq`] - checks that argument's reference points to the same location as provided
//!    reference. Compares references returned by [`std::ops::Deref::deref`] of `T`. Usage example:
//! ```
//! # use automock::*;
//! # use std::rc::Rc;
//! #[mock]
//! trait Trait {
//!     fn work(&self, r: Rc<i32>) -> i32;
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! let r1 = Rc::new(1);
//! let r2 = r1.clone();
//! mock.setup().work(Arg::ref_eq(r1.clone())).always_returns(10);
//!
//! // Act
//! let first  = mock.work(r1.clone());
//! let second = mock.work(r2);
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 10);
//! mock.received().work(Arg::ref_eq(r1), 2.times());
//! # }
//! ```
//! 5. [`Arg::ref_not_eq`] - checks that argument's reference DOES NOT point to the same place as
//!    provided reference. Compares references returned by [`std::ops::Deref::deref`] of `T`.
//!    Opposite of `Arg::ref_eq`.
//!
//! ## Controlling function behavior
//!
//! Calling `setup()` returns [`FnConfigurator`] - type that is used to tell mocked
//! function what it should do upon receiving matching call.
//!
//! ### Setting return values
//! Mocked functions that have return values can set them in multiple ways:
//! 1. [`FnConfigurator::returns`] - sets single-use return value.
//! 2. [`FnConfigurator::returns_many`] - sets multiple single-use return values in one call.
//! 3. [`FnConfigurator::always_returns`] - sets return value that can be returned unlimited number
//!    of times.
//! 4. [`FnConfigurator::returns_with`] - calculates return value on the fly using function's source
//!    arguments. Receives tuple of argument references. Returns unlimited number of times.
//!
//! If multiple return values were specified they will be used in the same order:
//! ```
//! use automock::*;
//!
//! #[mock] fn work(v: i32) -> i32 { v + 1 }
//!
//! # fn main() {
//! // Arrange
//! work::setup(Arg::Any).returns(10)                   // config #1
//!      .setup(Arg::Any).returns_many([20, 30])        // config #2
//!      .setup(Arg::Any).returns_with(|(v,)| *v + 10); // config #3
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
//! arguments.
//!
//! If the function has a return value, its callback can be configured using
//! [`FnCallbackConfigurator::and_does`] after its return value was specified. If function does not
//! have return value callback can be set by calling [`FnConfigurator::does`] straightaway. Static
//! functions receive tuple of argument values in the callback:
//! ```
//! use automock::*;
//!
//! #[mock] fn get(v: i32) -> i32 { v + 1 }
//! #[mock] fn set(v: i32) {}
//!
//! # fn main() {
//! get::setup(Arg::Any)    // Function with return value
//!     .returns(10)        // must set return value first
//!     .and_does(|(v,)| assert_eq!(*v, 10));
//!
//!                         // Function without return value
//! set::setup(Arg::Any)    // can set callback immediately
//!     .does(|(v,)| assert_eq!(*v, 20));
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
//!     .get().returns(10)
//!     .work(Arg::Any)
//!     .does(|mock_ref, (v,)| {
//!         assert_eq!(mock_ref.get(), 10);
//!         assert_eq!(*v, 20);
//!     });
//! mock.work(20);
//! # }
//! ```
//!
//! ## Base implementation
//!
//! Mocked functions can use their base implementation in tests. To do so, apply `#[mock]` attribute
//! with `base` argument: `#[mock(base)]`.
//! ```
//! # use automock::*; #[mock] struct Struct;
//! #[mock(base)] trait Trait {}
//! #[mock(base)] impl Struct {}
//! #[mock(base)] fn function() {}
//! # fn main() {}
//! ```
//!
//! To tell mock object to use base implementation call `call_base()` on function setup:
//!
//! ```
//! use automock::*;
//!
//! #[mock] fn dependency() {}
//! #[mock(base)]
//! fn work() {
//!     dependency();   // will be called in test
//! }
//!
//! # fn main() {
//! // Arrange
//! work::setup().call_base();
//!
//! // Act
//! work();
//!
//! // Assert
//! dependency::received(1.time());
//! # }
//! ```
//!
//! Functions that have return values treat `call_base()` as return value configuration, so you can
//! not call any of `returns` functions after enabling base implementation:
//!
//! ```compile_fail
//! # use automock::*;
//! # [mock(base)] fn work() -> i32 { 1 }
//!
//! work::setup().call_base().returns(10);
//!                        // ^^^^^^^ - error, return value is already
//!                        //           provided by base implementation
//! ```
//!
//! Base implementation usage is completely optional, you can mix mocked behavior with base calls.
//!
//! In traits, only functions with default implementation can use `call_base()`:
//! ```
//! use automock::*;
//!
//! #[mock(base)]
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
//!     .work(20).returns(30);
//!
//! // Act
//! let first  = mock.work(10);
//! let second = mock.work(20);
//!
//! // Assert
//! assert_eq!(first,  11);
//! assert_eq!(second, 30);
//! mock.received()
//!     .work(10, 1.time())
//!     .work(20, 1.time())
//!     .dependency(1.time());
//! # }
//! ```
//!
//! ### Limitations
//!
//! TODO - this limitation is no longer valid, fix docs (basically now calls that were forwarded to
//! base implementation won't be registered by mock and they can't be inspected by `received`)
//! There is one limitation: all arguments of function must implement [`Clone`] for its base
//! implementation to be used in tests. If even single argument does not implement `Clone` you will
//! get compilation error and will have to change your code or just use `#[mock]`:
//! ```compile_fail
//! # use automock::*;
//! struct Unclonable;
//! #[mock(base)]
//! fn work(_: Unclonable) {}
//!
//! # fn main() {}
//! ```
//!
//! ## Verifying calls
//!
//! You can verify exactly how a mocked function was called. To do it, use `received()` followed by
//! descriptions of expected calls:
//! ```
//! use automock::*;
//!
//! #[mock] fn work() {}
//!
//! # fn main() {
//! work();
//! work::received(1.time());   // checks that `work` was called one time
//! # }
//! ```
//!
//! You can verify exact values of passed arguments using [`Arg`], for more information see
//! [Arguments matching](#arguments-matching). Here's simple example that checks that mocked
//! function was called with specific argument:
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
//!
//! // Act
//! work(number, string);
//!
//! // Assert
//! work::received(
//!     Arg::is(|actual_number: &i32| *actual_number == number),
//!     Arg::not_eq("veridis quo"),
//!     1.time());
//! # }
//! ```
//!
//! ### Verify number of calls
//!
//! The number of calls is verified using [`Times`] type. It can be defined in several ways:
//! 1. [`Times::Never`] - expects function to never be called with given arguments.
//! 2. [`Times::Once`] and [`Times::Exactly`]`(N)` - expects function to be called exactly once or
//!    `N` times respectively.
//! 3. [`AsTimes::time`] and [`AsTimes::times`] - syntactic sugar for constructing
//!    [`Times::Exactly`]`(N)` from `usize` values like `1.time()` or `2.times()`.
//!
//! ### Verify no other calls were performed
//!
//! You can also check that no calls except the ones you expected were called using
//! `no_other_calls()`:
//! ```
//! use automock::*;
//!
//! #[mock] fn set(_: i32) {}
//!
//! # fn main() {
//! // Act
//! set(10);
//! set(20);
//!
//! // Assert
//! set::received(10, 1.time())
//!     .received(20, 1.time())
//!     .no_other_calls();  // checks that no other calls were performed
//! # }
//! ```
//!
//! If there were some unvalidated calls when `no_other_calls()` was called, then this function will
//! panic:
//! ```should_panic
//! # use automock::*;
//! #[mock]
//! trait Trait { fn set(&self, _: i32); }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//!
//! // Act
//! mock.set(10);
//! mock.set(20);
//! mock.set(30);
//!
//! // Assert
//! mock.received()
//!     .set(10, 1.time())
//!     .set(20, 1.time())
//!     .no_other_calls();  // will panic because `set(30)` was not validated
//! # }
//! ```
//!
//! ### Verify no calls were performed in static functions
//!
//! For trait and structure mocks, you can call `mock.received().no_other_calls()` to verify that
//! they received no other calls. For static functions you need to use `received_nothing()`
//! function:
//! ```
//! use automock::*;
//!
//! #[mock] fn work() {}
//!
//! # fn main() {
//! // Assert
//! work::received_nothing();
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
//! mock.setup().get().returns(10);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 10);
//! mock.received().get(1.time());
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
//! #[mock(base)]
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
//! let mut mock = Struct::new::<str>(10i32, &[1, 2, 3], "quo vadis");
//! mock.setup().get_t1().returns(20);
//!
//! // Act
//! let result = mock.get_t1();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.received().get_t1(1.time());
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
//!     .returns("quo vadis");
//! get::setup::<[u8; 3], &'static str>([1, 2, 3])
//!     .returns("veridis quo")
//!     .setup([10, 10, 10])        // generic types are inferred here by `get::setup` function,
//!     .returns("third call");     // no need to specify them again
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
//! get::received::<_, &str>(10i32, 1.time());      // type inference also works with mocks
//! get::received::<_, &str>([1u8, 2, 3], 1.time())
//!     .received([10u8, 10, 10], 1.time());
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
//! #[mock(base)]
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
//!     .get_item().returns(10)
//!     .get_number().call_base();
//!
//! // Act
//! let item = mock.get_item();
//! let number = mock.get_number();
//!
//! // Assert
//! assert_eq!(item, 10);
//! assert_eq!(number, 20);
//! mock.received()
//!     .get_item(1.time())
//!     .get_number(1.time());
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
//! work::setup(Arg::Any).returns(Box::new(20));
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
//! #[mock(base)]
//! # #[allow(improper_ctypes_definitions)]
//! async unsafe extern "C" fn modifiers() {
//!     async_dep().await;
//!     unsafe_dep();
//!     extern_dep();
//! }
//!
//! #[mock(base)]
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
//! To verify that calls were received in the specific order, wrap `received()` assertions in a
//! [`verify_call_order`] callback. This function checks that all calls inside of it were asserted
//! sequentially:
//! ```
//! use automock::*;
//!
//! #[mock] fn set(_: i32) {}
//!
//! # fn main() {
//! // Act
//! set(1);
//! set(2);
//! set(3);
//!
//! // Assert
//! verify_call_order(|| {
//!     set::received(1, 1.time())
//!         .received(2, 1.time())
//!         .received(3, 1.time());
//! });
//! # }
//! ```
//!
//! If call order was violated [`verify_call_order`] will panic after performing all `received`
//! assertions. For example, this panics:
//! ```should_panic
//! # use automock::*; #[mock] fn set(_: i32) {}
//! # fn main() {
//! // Act
//! set(2);
//! set(1);
//! set(3);
//!
//! // Assert
//! verify_call_order(|| {
//!     set::received(1, 1.time())
//!         .received(2, 1.time())
//!         .received(3, 1.time());
//! }); // panic: expected order 1-2-3, actually got 2-1-3
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
//! #[mock(base)]
//! impl Struct {
//!     pub fn new() -> Self { Self }
//!     pub fn third(&self) {}
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut trait_mock = TraitMock::new();
//! let mut struct_mock = Struct::new();
//!
//! // Act
//! first();
//! trait_mock.second();
//! struct_mock.third();
//!
//! // Assert
//! verify_call_order(|| {
//!     first::received(1.time());
//!     trait_mock.received().second(1.time());
//!     struct_mock.received().third(1.time());
//! });
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
//! mock.setup().boxed().returns(10);   // setup() and received() are always called on "&mut self"
//!
//! // Act
//! let result = Box::new(mock.clone()).boxed();    // another feature - mocks cloning, more
//!                                                 // information in "Cloning mocks" section
//!
//! // Assert
//! assert_eq!(result, 10);
//! mock.received().boxed(1.time());
//! # }
//! ```
//!
//! ## Cloning mocks
//!
//! Mock can be cloned either if:
//! 1. it is mock of a trait,
//! 2. it is mock of a struct that has `#[derive(Clone)]` attribute (manually implementing [`Clone`]
//!    won't work).
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
//!
//! // Act
//! mock.consume();
//!
//! // Assert
//! mock_for_verification.received().consume(1.time());
//! # }
//! ```
//!
//! ## Crate settings
//!
//! You can configure some crate level options using [`read_settings`] and [`write_settings`]
//! functions (or use [`SETTINGS`] static variable directly). This config contains infrastructure
//! settings.  
//! Currently it has only `max_invalid_calls_listed_count` parameter that controls how many invalid
//! calls will be listed in case of a calls validation error.
//!
//! Settings can be used like this:
//!
//! ```should_panic
//! use automock::*;
//!
//! #[mock] fn work(_: i32) {}
//!
//! # fn main() {
//! // Arrange
//! write_settings().max_invalid_calls_listed_count = 3;
//!
//! // Act
//! for i in 0..10 {
//!     work(i);
//! }
//!
//! // Assert
//! work::received_nothing(); // will panic and show only first 3 calls,
//!                           // other calls won't be listed
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
//! build and one for `test` that tracks calls:
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
//! This is basically what `automock` does - it automatically creates infrastructure for mocking,
//! except that it generates a more complex code for flexible configuration.
//!
//! # Undefined behavior
//!
//! `automock` infrastructure stores all call arguments in mock objects so that they can later be
//! inspected by `received()`. When a mocked function receives a reference, this can cause a
//! dangling reference access if the referenced value is dropped before `received()` is called.
//! This can result in undefined behavior.  
//! Here's an example of UB:
//! ```should_panic
//! # use automock::*;
//! #[mock]
//! fn work(_: &i32) {}
//!
//! fn use_work() {
//!     let local = 10;
//!     work(&local);
//! }
//!
//! # fn main() {
//! // Act
//! use_work();
//!
//! // Assert
//! work::received(Arg::is(|r: &&i32| **r == 10), 1.time());
//!                                 // ^^^
//!                                 // UB - `r` stores reference to `local` dropped in `use_work`
//! # }
//! ```
//!
//! To fix this you may need to refactor your function. In example above hard-coded reference can be
//! passed as argument instead:
//! ```
//! # use automock::*;
//! # #[mock] fn work(_: &i32) {}
//! fn use_work(r: &i32) {
//!     work(r);
//! }
//!
//! # fn main() {
//! // Arrange
//! let test_local = 10;
//!
//! // Act
//! use_work(&test_local);
//!
//! // Assert
//! // receives reference to `test_local` that is still alive in this scope
//! work::received(Arg::is(|r: &&i32| **r == test_local), 1.time());
//! # }
//! ```
//! Rule of thumb is that if function that accepts references produces flaky results, this is most
//! likely because it receives dangling reference.
pub use automock_proc_macro::mock;

#[doc(hidden)]
pub use automock_core::args::*;
pub use automock_core::infrastructure::{FnCallbackConfigurator, FnConfigurator, Mockable};
pub use automock_core::settings::*;
pub use automock_core::times::*;
pub use automock_core::transmute_lifetime;
pub use automock_core::verify_call_order;

#[doc(hidden)]
pub mod for_generated {
    pub use automock_core::args::*;
    pub use automock_core::fn_parameters::*;
    pub use automock_core::infrastructure::*;
    pub use automock_core::times::*;
    pub use automock_core::*;
}
