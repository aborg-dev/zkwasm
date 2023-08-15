(module
 (import "env" "assert" (func $assert (param i32)))
 (func $main
	(local $x i32)
	(local.set $x (i32.const 2))
	(local.get $x)
	(i32.const 2)
	(i32.eq)
	call $assert)
(start $main))
