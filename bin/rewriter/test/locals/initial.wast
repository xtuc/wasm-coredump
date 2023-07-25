(module
  (memory 10)
  (func $entry (param i32 i64 f32 f64) (result i32)
    (local i32 i64 f32 f64)
    (unreachable)
  )
  (export "entry" (func $entry))
  (export "memory" (memory 0))
)
