(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	f32.const 4
	f32.sqrt
	i32.trunc_f32_s
	f32.const 2
	i32.trunc_f32_s
	call $assert_eq)
 (start $main))
