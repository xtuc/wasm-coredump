(module
  (memory 10)
  (global $one i32 (i32.const 1))
  (global $second i64 (i64.const 2))
  (func $entry (param i32 i32) (result i32)
    (local.get 0)
    (local.get 1)
    (call $addTwo)
  )
  (func $addTwo (param i32 i32) (result i32)
    (local.get 0)
    (local.get 1)
    (i32.add)
    (unreachable)
  )
  (export "addTwo" (func $entry))
  (export "memory" (memory 0))
)
