(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $fib (param $n i32) (result i32)
	(if (result i32)
	 (i32.eqz (local.get $n))
	 (then (i32.const 0))
	 (else
			(if (result i32)
			 (i32.eqz (i32.sub (local.get $n) (i32.const 1)))
			 (then (i32.const 1))
			 (else
			 	(call $fib (i32.sub (local.get $n) (i32.const 1)))
			 	(call $fib (i32.sub (local.get $n) (i32.const 2)))
				(i32.add))
			)
	)))

 (func $main
	;; 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89
	(call $fib (i32.const 11))
	(i32.const 89)
	call $assert_eq)
(start $main))
