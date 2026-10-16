use automock::*;

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
        fn_data.verify_calls()
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

#[mock]
trait Trait {
    fn work(&self, v: i32) -> i32;
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
        let result = 20;
        mock.setup().work(v).returns(result);
        
        // TODO - target API?
        // mock.setup()
        //     .work(1).returns_once(2)
        //     .work(3).returns(4, 5.times());

        // Act
        let actual_result = mock.work(v);

        // Assert
        assert_eq!(actual_result, result);

        mock.received().work(v, 1.time());
    }
}
