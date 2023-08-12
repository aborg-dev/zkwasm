(module
  (import "env" "assert" (func $assert (param i32)))
  (func $main
		i32.const 2
		i32.const 3
		i32.add
		i32.const 5
		i32.eq
		call $assert)
	(start $main))
