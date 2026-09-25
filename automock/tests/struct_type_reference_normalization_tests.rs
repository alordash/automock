use automock::*;

mod unit_mod {
    use super::*;

    #[mock]
    pub struct Unit;

    #[mock(base)]
    impl Unit {
        #[allow(unused)]
        fn ident(&self) {
            let unit = Unit;
            let Unit = unit;
            let s = Self;
            let Self = s;
        }

        fn static_ident() {
            let unit = Unit;
            let Unit = unit;
            let s = Self;
            let Self = s;
        }
    }
}

#[mock(base)]
impl unit_mod::Unit {
    fn path(&self) {
        let unit = unit_mod::Unit;
        let unit_mod::Unit = unit;
        let s = Self;
        let Self = s;
    }

    fn static_path() {
        let unit = unit_mod::Unit;
        let unit_mod::Unit = unit;
        let s = Self;
        let Self = s;
    }
}

mod named_mod {
    use super::*;

    #[mock]
    pub struct Named {
        #[allow(unused)]
        pub v: i32,
    }

    #[mock(base)]
    impl Named {
        #[allow(unused)]
        fn ident(&self) {
            let unit = Named { v: 1 };
            let Named {
                #[allow(unused_variables)]
                    v: a,
            } = unit;
            let s = Self { v: 2 };
            let Self = s;
        }

        fn static_ident() {
            let unit = Named { v: 1 };
            let Named {
                #[allow(unused_variables)]
                    v: a,
            } = unit;
            let s = Self { v: 2 };
            let Self = s;
        }
    }
}

#[mock(base)]
impl named_mod::Named {
    fn path(&self) {
        let unit = named_mod::Named { v: 1 };
        let named_mod::Named {
            #[allow(unused_variables)]
                v: a,
        } = unit;
        let s = Self { v: 2 };
        let Self = s;
    }

    fn static_path() {
        let unit = named_mod::Named { v: 1 };
        let named_mod::Named {
            #[allow(unused_variables)]
                v: a,
        } = unit;
        let s = Self { v: 2 };
        let Self = s;
    }
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn compile() {
        let _ = unit_mod::Unit {
            __am_data: Default::default(),
        };
    }
}
