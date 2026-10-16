// #[mock]
// fn work(v: i32) -> i32 {
//     v + 1
// }

#[allow(clippy::useless_transmute)]
fn work(v: i32) -> i32 {
    use work::*;
    let call = work_Call::<'_> {
        __rs_generics: ::core::marker::PhantomData,
        v: ::automock::transmute_lifetime!(v),
    };
    let fn_data: &::automock::for_generated::FnData<'_, workMock<'_>, true, true, false> =
        ::automock::for_generated::get_static_fn_data("work");
    fn_data.handle_base((), call, __am_base_work)
}
#[allow(clippy::all)]
#[allow(unused)]
#[allow(private_interfaces)]
#[allow(private_bounds)]
#[allow(unreachable_pub)]
#[allow(nonstandard_style)]
mod work {
    use super::*;
    use automock_core::infrastructure::paaanic;
    pub fn __am_base_work(_: (), call: work_Call<'_>) -> i32 {
        let work_Call::<'_> { v, .. } = call;
        let v: i32 = ::automock::transmute_lifetime!(v);
        v + 1
    }
    pub fn setup<'__aml>(
        v: impl ::automock::for_generated::IntoArg<i32>,
    ) -> ::automock::for_generated::FnConfigurator<
        '__aml,
        workMock<'__aml>,
        workStaticSetup<'__aml>,
        (&'__aml mut i32,),
        i32,
        workMock<'__aml>,
        true,
        true,
        false,
    > {
        ::automock::for_generated::clear_static_fn_data::<workMock<'__aml>>();
        workStaticSetup::<'__aml> {
            __rs_generics: ::core::marker::PhantomData,
        }
        .setup(v)
    }
    pub fn verify_calls<'__aml>() {
        let fn_data: &::automock::for_generated::FnData<'_, workMock<'__aml>, true, true, false> =
            ::automock::for_generated::get_static_fn_data("work");
        let verify_calls_result = fn_data.verify_calls();
        paaanic(vec![verify_calls_result]);
    }
    pub fn received<'__aml>(
        v: impl ::automock::for_generated::IntoArg<i32>,
        __ts: ::automock::for_generated::Times,
    ) -> ::automock::for_generated::ArgRefsBinder<workStaticReceived<'__aml>, (&'__aml mut i32,)>
    {
        workStaticReceived::<'__aml> {
            __rs_generics: ::core::marker::PhantomData,
        }
        .received(v, __ts)
    }
    pub fn received_nothing<'__aml>() {
        ::automock::for_generated::verify_static_fn_received_nothing_else::<workMock<'_>>()
    }
    pub struct work_Call<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), i32)>,
        pub(super) v: i32,
    }
    impl<'__aml> ::automock::for_generated::IGenericsInfoProvider for work_Call<'__aml> {}
    impl<'__aml> ::automock::for_generated::ICall for work_Call<'__aml> {
        fn get_arg_infos(&self) -> Vec<::automock::for_generated::ArgInfo> {
            use ::automock::for_generated::arg_printing::*;
            vec![::automock::for_generated::ArgInfo::new(
                "v",
                &self.v,
                (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                    &self.v, &i32
                )))
                    .debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.v,))) as *mut _ as *mut ()
        }
    }
    struct work_ArgsChecker<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), i32)>,
        v: ::automock::for_generated::Arg<i32>,
    }
    impl<'__aml> ::automock::for_generated::IGenericsInfoProvider for work_ArgsChecker<'__aml> {}
    impl<'__aml> ::automock::for_generated::IArgsChecker for work_ArgsChecker<'__aml> {
        fn check(
            &self,
            dyn_call: &::automock::for_generated::DynCall<'_>,
        ) -> Vec<::automock::for_generated::ArgCheckResult> {
            use ::automock::for_generated::arg_printing::*;
            let call: &work_Call<'__aml> = dyn_call.downcast_to();
            vec![
                ::automock::transmute_lifetime!(&self.v, &::automock::for_generated::Arg::<i32>)
                    .check(
                        "v",
                        ::automock::transmute_lifetime!(&call.v),
                        (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                            &call.v, &i32
                        )))
                            .debug_string(),
                    ),
            ]
        }
        fn fmt_args(&self) -> String {
            use ::automock::for_generated::arg_printing::*;
            format!(
                "{}",
                (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                    &&self.v,
                    &&::automock::for_generated::Arg::<i32>
                )))
                    .debug_string()
            )
        }
    }
    pub struct workMock<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), i32)>,
    }
    pub struct workStaticSetup<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), i32)>,
    }
    impl<'__aml> workStaticSetup<'__aml> {
        pub fn setup(
            &self,
            v: impl ::automock::for_generated::IntoArg<i32>,
        ) -> ::automock::for_generated::FnConfigurator<
            '_,
            workMock<'__aml>,
            Self,
            (&'__aml mut i32,),
            i32,
            workMock<'__aml>,
            true,
            true,
            false,
        > {
            let args_checker = work_ArgsChecker::<'__aml> {
                __rs_generics: ::core::marker::PhantomData,
                v: ::automock::transmute_lifetime!(v.into_arg(|v| {
                    use ::automock::for_generated::arg_printing::*;
                    (&::automock::for_generated::ArgPrinter(v)).debug_string()
                })),
            };
            let fn_data: &::automock::for_generated::FnData<
                '_,
                workMock<'__aml>,
                true,
                true,
                false,
            > = ::automock::for_generated::get_static_fn_data("work");
            let fn_configurator: ::automock::for_generated::FnConfigurator<
                '_,
                workMock<'__aml>,
                Self,
                (&'__aml mut i32,),
                i32,
                workMock<'__aml>,
                true,
                true,
                false,
            > = fn_data.add_config(args_checker, self);
            ::automock::transmute_lifetime!(fn_configurator)
        }
    }
    pub struct workStaticReceived<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), i32)>,
    }
    impl<'__aml> ::core::clone::Clone for workStaticReceived<'__aml> {
        #[inline]
        fn clone(&self) -> workStaticReceived<'__aml> {
            workStaticReceived::<'__aml> {
                __rs_generics: ::core::clone::Clone::clone(&self.__rs_generics),
            }
        }
    }
    impl<'__aml> workStaticReceived<'__aml> {
        pub fn received(
            &self,
            v: impl ::automock::for_generated::IntoArg<i32>,
            __ts: ::automock::for_generated::Times,
        ) -> ::automock::for_generated::ArgRefsBinder<Self, (&'__aml mut i32,)> {
            let args_checker = work_ArgsChecker::<'__aml> {
                __rs_generics: ::core::marker::PhantomData,
                v: ::automock::transmute_lifetime!(v.into_arg(|v| {
                    use ::automock::for_generated::arg_printing::*;
                    (&::automock::for_generated::ArgPrinter(v)).debug_string()
                })),
            };
            let fn_data: &::automock::for_generated::FnData<'_, workMock<'_>, true, true, false> =
                ::automock::for_generated::get_static_fn_data("work");
            fn_data.verify_received(args_checker, __ts);
            automock::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn no_other_calls(&self) {
            ::automock::for_generated::verify_static_fn_received_nothing_else::<workMock<'_>>()
        }
    }
}

// #[mock]
// trait Trait {
//     fn foo(&self, v: i32) -> i32;
//     fn bar(&self, b: bool) -> bool;
// }

trait Trait {
    fn foo(&self, v: i32) -> i32;
    fn bar(&self, b: bool) -> bool;
}
#[allow(unused)]
use __automock_generated_TraitMock::TraitMock;
#[allow(clippy::all)]
#[allow(unused)]
#[allow(private_interfaces)]
#[allow(private_bounds)]
#[allow(unreachable_pub)]
#[allow(nonstandard_style)]
mod __automock_generated_TraitMock {
    use super::*;
    pub struct foo_Call<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), i32)>,
        pub(super) v: i32,
    }
    impl<'__aml> ::automock::for_generated::IGenericsInfoProvider for foo_Call<'__aml> {}
    impl<'__aml> ::automock::for_generated::ICall for foo_Call<'__aml> {
        fn get_arg_infos(&self) -> Vec<::automock::for_generated::ArgInfo> {
            use ::automock::for_generated::arg_printing::*;
            vec![::automock::for_generated::ArgInfo::new(
                "v",
                &self.v,
                (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                    &self.v, &i32
                )))
                    .debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.v,))) as *mut _ as *mut ()
        }
    }
    struct foo_ArgsChecker<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), i32)>,
        v: ::automock::for_generated::Arg<i32>,
    }
    impl<'__aml> ::automock::for_generated::IGenericsInfoProvider for foo_ArgsChecker<'__aml> {}
    impl<'__aml> ::automock::for_generated::IArgsChecker for foo_ArgsChecker<'__aml> {
        fn check(
            &self,
            dyn_call: &::automock::for_generated::DynCall<'_>,
        ) -> Vec<::automock::for_generated::ArgCheckResult> {
            use ::automock::for_generated::arg_printing::*;
            let call: &foo_Call<'__aml> = dyn_call.downcast_to();
            vec![
                ::automock::transmute_lifetime!(&self.v, &::automock::for_generated::Arg::<i32>)
                    .check(
                        "v",
                        ::automock::transmute_lifetime!(&call.v),
                        (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                            &call.v, &i32
                        )))
                            .debug_string(),
                    ),
            ]
        }
        fn fmt_args(&self) -> String {
            use ::automock::for_generated::arg_printing::*;
            format!(
                "{}",
                (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                    &&self.v,
                    &&::automock::for_generated::Arg::<i32>
                )))
                    .debug_string()
            )
        }
    }
    pub struct bar_Call<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), bool)>,
        pub(super) b: bool,
    }
    impl<'__aml> ::automock::for_generated::IGenericsInfoProvider for bar_Call<'__aml> {}
    impl<'__aml> ::automock::for_generated::ICall for bar_Call<'__aml> {
        fn get_arg_infos(&self) -> Vec<::automock::for_generated::ArgInfo> {
            use ::automock::for_generated::arg_printing::*;
            vec![::automock::for_generated::ArgInfo::new(
                "b",
                &self.b,
                (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                    &self.b, &bool
                )))
                    .debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.b,))) as *mut _ as *mut ()
        }
    }
    struct bar_ArgsChecker<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (), bool)>,
        b: ::automock::for_generated::Arg<bool>,
    }
    impl<'__aml> ::automock::for_generated::IGenericsInfoProvider for bar_ArgsChecker<'__aml> {}
    impl<'__aml> ::automock::for_generated::IArgsChecker for bar_ArgsChecker<'__aml> {
        fn check(
            &self,
            dyn_call: &::automock::for_generated::DynCall<'_>,
        ) -> Vec<::automock::for_generated::ArgCheckResult> {
            use ::automock::for_generated::arg_printing::*;
            let call: &bar_Call<'__aml> = dyn_call.downcast_to();
            vec![
                ::automock::transmute_lifetime!(&self.b, &::automock::for_generated::Arg::<bool>)
                    .check(
                        "b",
                        ::automock::transmute_lifetime!(&call.b),
                        (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                            &call.b, &bool
                        )))
                            .debug_string(),
                    ),
            ]
        }
        fn fmt_args(&self) -> String {
            use ::automock::for_generated::arg_printing::*;
            format!(
                "{}",
                (&::automock::for_generated::ArgPrinter(::automock::transmute_lifetime!(
                    &&self.b,
                    &&::automock::for_generated::Arg::<bool>
                )))
                    .debug_string()
            )
        }
    }
    pub struct TraitMock<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (),)>,
        pub __mock_data: ::automock::for_generated::SharedMockData,
    }
    impl<'__aml> ::core::clone::Clone for TraitMock<'__aml> {
        #[inline]
        fn clone(&self) -> TraitMock<'__aml> {
            TraitMock::<'__aml> {
                __rs_generics: ::core::clone::Clone::clone(&self.__rs_generics),
                __mock_data: ::core::clone::Clone::clone(&self.__mock_data),
            }
        }
    }
    impl<'__aml> Trait for TraitMock<'__aml> {
        fn foo(&self, v: i32) -> i32 {
            let call = foo_Call::<'_> {
                __rs_generics: ::core::marker::PhantomData,
                v: ::automock::transmute_lifetime!(v),
            };
            let fn_data: &::automock::for_generated::FnData<'_, TraitMock<'_>, true, false, true> =
                self.__mock_data.get_fn_data(
                    "Trait",
                    "foo",
                    ::automock::for_generated::IGenericsInfoProvider::get_generics_hash_key(&call),
                );
            fn_data.handle(self, call)
        }
        fn bar(&self, b: bool) -> bool {
            let call = bar_Call::<'_> {
                __rs_generics: ::core::marker::PhantomData,
                b: ::automock::transmute_lifetime!(b),
            };
            let fn_data: &::automock::for_generated::FnData<'_, TraitMock<'_>, true, false, true> =
                self.__mock_data.get_fn_data(
                    "Trait",
                    "bar",
                    ::automock::for_generated::IGenericsInfoProvider::get_generics_hash_key(&call),
                );
            fn_data.handle(self, call)
        }
    }
    impl<'__aml> TraitMock<'__aml> {
        pub fn id(&self) -> usize {
            self.__mock_data.id()
        }
        pub fn new() -> Self {
            Self {
                __rs_generics: ::core::marker::PhantomData,
                __mock_data: ::core::default::Default::default(),
            }
        }
        pub fn setup(&mut self) -> TraitSetup<'__aml> {
            TraitSetup::<'__aml> {
                __rs_generics: ::core::marker::PhantomData,
                __mock_data: self.__mock_data.clone(),
            }
        }
        pub fn received(&mut self) -> TraitReceived<'__aml> {
            TraitReceived::<'__aml> {
                __rs_generics: ::core::marker::PhantomData,
                __mock_data: self.__mock_data.clone(),
            }
        }
        pub fn verify_calls(&mut self) {
            self.__mock_data.verify_calls();
        }
    }
    pub struct TraitSetup<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (),)>,
        pub __mock_data: ::automock::for_generated::SharedMockData,
    }
    impl<'__aml> TraitSetup<'__aml> {
        pub fn foo(
            &self,
            v: impl ::automock::for_generated::IntoArg<i32>,
        ) -> ::automock::for_generated::FnConfigurator<
            '_,
            TraitMock<'__aml>,
            Self,
            (&'__aml mut i32,),
            i32,
            &TraitMock<'__aml>,
            true,
            false,
            true,
        > {
            let args_checker = foo_ArgsChecker::<'__aml> {
                __rs_generics: ::core::marker::PhantomData,
                v: ::automock::transmute_lifetime!(v.into_arg(|v| {
                    use ::automock::for_generated::arg_printing::*;
                    (&::automock::for_generated::ArgPrinter(v)).debug_string()
                })),
            };
            let fn_data: &::automock::for_generated::FnData<
                '_,
                TraitMock<'__aml>,
                true,
                false,
                true,
            > = self.__mock_data.get_fn_data(
                "Trait",
                "foo",
                ::automock::for_generated::IGenericsInfoProvider::get_generics_hash_key(
                    &args_checker,
                ),
            );
            let fn_configurator: ::automock::for_generated::FnConfigurator<
                '_,
                TraitMock<'__aml>,
                Self,
                (&'__aml mut i32,),
                i32,
                &TraitMock<'__aml>,
                true,
                false,
                true,
            > = fn_data.add_config(args_checker, self);
            ::automock::transmute_lifetime!(fn_configurator)
        }
        pub fn bar(
            &self,
            b: impl ::automock::for_generated::IntoArg<bool>,
        ) -> ::automock::for_generated::FnConfigurator<
            '_,
            TraitMock<'__aml>,
            Self,
            (&'__aml mut bool,),
            bool,
            &TraitMock<'__aml>,
            true,
            false,
            true,
        > {
            let args_checker = bar_ArgsChecker::<'__aml> {
                __rs_generics: ::core::marker::PhantomData,
                b: ::automock::transmute_lifetime!(b.into_arg(|v| {
                    use ::automock::for_generated::arg_printing::*;
                    (&::automock::for_generated::ArgPrinter(v)).debug_string()
                })),
            };
            let fn_data: &::automock::for_generated::FnData<
                '_,
                TraitMock<'__aml>,
                true,
                false,
                true,
            > = self.__mock_data.get_fn_data(
                "Trait",
                "bar",
                ::automock::for_generated::IGenericsInfoProvider::get_generics_hash_key(
                    &args_checker,
                ),
            );
            let fn_configurator: ::automock::for_generated::FnConfigurator<
                '_,
                TraitMock<'__aml>,
                Self,
                (&'__aml mut bool,),
                bool,
                &TraitMock<'__aml>,
                true,
                false,
                true,
            > = fn_data.add_config(args_checker, self);
            ::automock::transmute_lifetime!(fn_configurator)
        }
    }
    pub struct TraitReceived<'__aml> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__aml (),)>,
        pub __mock_data: ::automock::for_generated::SharedMockData,
    }
    impl<'__aml> ::core::clone::Clone for TraitReceived<'__aml> {
        #[inline]
        fn clone(&self) -> TraitReceived<'__aml> {
            TraitReceived::<'__aml> {
                __rs_generics: ::core::clone::Clone::clone(&self.__rs_generics),
                __mock_data: ::core::clone::Clone::clone(&self.__mock_data),
            }
        }
    }
    impl<'__aml> TraitReceived<'__aml> {
        pub fn foo(
            &self,
            v: impl ::automock::for_generated::IntoArg<i32>,
            __ts: ::automock::for_generated::Times,
        ) -> ::automock::for_generated::ArgRefsBinder<Self, (&'__aml mut i32,)> {
            let args_checker = foo_ArgsChecker::<'__aml> {
                __rs_generics: ::core::marker::PhantomData,
                v: ::automock::transmute_lifetime!(v.into_arg(|v| {
                    use ::automock::for_generated::arg_printing::*;
                    (&::automock::for_generated::ArgPrinter(v)).debug_string()
                })),
            };
            let fn_data: &::automock::for_generated::FnData<'_, TraitMock<'_>, true, false, true> =
                self.__mock_data.get_fn_data(
                    "Trait",
                    "foo",
                    ::automock::for_generated::IGenericsInfoProvider::get_generics_hash_key(
                        &args_checker,
                    ),
                );
            fn_data.verify_received(args_checker, __ts);
            automock::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn bar(
            &self,
            b: impl ::automock::for_generated::IntoArg<bool>,
            __ts: ::automock::for_generated::Times,
        ) -> ::automock::for_generated::ArgRefsBinder<Self, (&'__aml mut bool,)> {
            let args_checker = bar_ArgsChecker::<'__aml> {
                __rs_generics: ::core::marker::PhantomData,
                b: ::automock::transmute_lifetime!(b.into_arg(|v| {
                    use ::automock::for_generated::arg_printing::*;
                    (&::automock::for_generated::ArgPrinter(v)).debug_string()
                })),
            };
            let fn_data: &::automock::for_generated::FnData<'_, TraitMock<'_>, true, false, true> =
                self.__mock_data.get_fn_data(
                    "Trait",
                    "bar",
                    ::automock::for_generated::IGenericsInfoProvider::get_generics_hash_key(
                        &args_checker,
                    ),
                );
            fn_data.verify_received(args_checker, __ts);
            automock::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn no_other_calls(&self) {
            automock::for_generated::IMockData::verify_received_nothing_else(&self.__mock_data)
        }
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn static_fn_Ok() {
        // Arrange
        let v = 10;
        let result = 20;
        work::setup(v)
            .returns(result)
            .setup(v + 1)
            .returns_many([result + 1, 6, 8, 1])
            .setup(v + 2)
            .returns(result + 2)
            .setup(v + 1)
            .returns(result + 1);

        // TODO - target API?
        // work::setup(1).returns_once(2)
        //      .setup(3).returns(4, 5.times());

        // Act
        let actual_result = work(v);

        // Assert
        assert_eq!(actual_result, result);

        // work::received(v, 1.time());
        work::verify_calls();
    }

    #[test]
    fn object_mock_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        let v = 10;
        let b = false;
        let foo_result = 20;
        let bar_result = true;
        mock.setup()
            .foo(v)
            .returns(foo_result)
            .foo(102)
            .returns(202)
            .bar(b)
            .returns(bar_result)
            .bar(b)
            .returns(bar_result);

        // TODO - target API?
        // mock.setup()
        //     .work(1).returns_once(2)
        //     .work(3).returns(4, 5.times());

        // Act
        let actual_foo_result = mock.foo(v);
        let actual_bar_result = mock.bar(b);

        // Assert
        assert_eq!(actual_foo_result, foo_result);
        assert_eq!(actual_bar_result, bar_result);

        mock.verify_calls();
    }
}
