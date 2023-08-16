#[cfg(test)]
mod tests {
    use wasmi::{Caller, Engine, Func, Linker, Module, Store};
    use zkwasm::codegen;

    fn test_module(name: &str) {
        let wat = wat::parse_file(format!("data/{name}.wat")).expect("Failed to parse WAT file");
        let engine = Engine::default();
        let module = Module::new(&engine, &wat[..]).unwrap();

        type HostState = ();
        let mut store = Store::new(&engine, ());
        let host_assert = Func::wrap(
            &mut store,
            |_caller: Caller<'_, HostState>, lhs: i32, rhs: i32| {
                assert_eq!(lhs, rhs);
            },
        );

        let mut linker = <Linker<HostState>>::new(&engine);
        linker.define("env", "assert_eq", host_assert).unwrap();
        let _ = linker
            .instantiate(&mut store, &module)
            .unwrap()
            .start(&mut store)
            .unwrap();

        let program = codegen::parse(&wat).unwrap();
        let expected = expect_test::expect_file![format!("../data/generated/{name}.zkasm")];
        expected.assert_eq(&program);
    }

    macro_rules! testcases {
        { $($name:ident,)* } => {
          $(
            #[test]
            fn $name() {
                test_module(stringify!($name));
            }
           )*
        };
    }

    testcases! {
        add,
        locals,
        locals_simple,
        counter,
        fibonacci,
    }
}
