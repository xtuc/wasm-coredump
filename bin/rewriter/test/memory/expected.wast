(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func))
  (type (;3;) (func))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (param i32)))
  (type (;6;) (func (param f32)))
  (type (;7;) (func (param f64)))
  (type (;8;) (func (param i64)))
  (func $valid (type 0) (result i32)
    (local i32)
    i32.const 0
    global.set 2
    i32.const 0
    i32.const 123
    i32.store
    i32.const 0
    local.tee 0
    memory.size
    i32.const 16384
    i32.mul
    i32.gt_u
    if  ;; label = @1
      call $coredump/write_coredump
      unreachable
    end
    local.get 0
    i32.load)
  (func $invalid (type 0) (result i32)
    (local i32)
    i32.const 1
    global.set 2
    i32.const 16392
    local.tee 0
    memory.size
    i32.const 16384
    i32.mul
    i32.gt_u
    if  ;; label = @1
      call $coredump/write_coredump
      unreachable
    end
    local.get 0
    i32.load)
  (func $invalid_with_locals (type 0) (result i32)
    (local i32 i64 i32)
    i32.const 2
    global.set 2
    i32.const 16392
    local.tee 2
    memory.size
    i32.const 16384
    i32.mul
    i32.gt_u
    if  ;; label = @1
      call $coredump/write_coredump
      unreachable
    end
    local.get 2
    i32.load)
  (func $invalid_nested (type 0) (result i32)
    (local i32)
    i32.const 16392
    local.tee 0
    memory.size
    i32.const 16384
    i32.mul
    i32.gt_u
    if  ;; label = @1
      call $coredump/unreachable_shim
      i32.const 3
      i32.const 0
      call $coredump/start_frame
      i32.const 667
      return
    end
    local.get 0
    i32.load)
  (func $invalid_nested1 (type 0) (result i32)
    call $invalid_nested
    global.get 1
    if  ;; label = @1
      i32.const 4
      i32.const 0
      call $coredump/start_frame
      global.get 2
      i32.const 4
      i32.eq
      if  ;; label = @2
        call $coredump/write_coredump
        unreachable
      else
        i32.const 667
        return
      end
    end)
  (func $invalid_nested2 (type 1)
    i32.const 5
    global.set 2
    call $invalid_nested1
    global.get 1
    if  ;; label = @1
      i32.const 5
      i32.const 0
      call $coredump/start_frame
      global.get 2
      i32.const 5
      i32.eq
      if  ;; label = @2
        call $coredump/write_coredump
        unreachable
      else
        return
      end
    end
    drop)
  (func $coredump/unreachable_shim (type 2)
    i32.const 1
    global.set 1)
  (func $coredump/write_coredump (type 3)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    i32.load
    i32.const 1836278016
    i32.eq
    if  ;; label = @1
      unreachable
    end
    global.get 0
    local.tee 3
    i32.const 16
    i32.add
    local.tee 0
    local.set 1
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 2
    i32.const 26
    i32.add
    i32.const 0
    local.get 3
    memory.copy
    i32.const 0
    i32.const 1836278016
    i32.store
    i32.const 4
    i32.const 1
    i32.store
    i32.const 8
    i32.const 0
    i32.store8
    local.get 0
    local.set 1
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 6
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=9
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 6
    local.get 1
    i32.const 127
    i32.and
    i32.store8 offset=9
    local.get 6
    i32.const 10
    i32.add
    local.set 1
    i32.const 9
    local.set 6
    loop  ;; label = @1
      local.get 6
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 5
        i32.add
        local.get 6
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        local.get 6
        i32.const 7
        i32.shr_u
        local.set 6
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 5
    i32.add
    local.get 6
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 5
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 99
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 111
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 114
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 101
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 115
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 116
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 97
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 99
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 107
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.get 1
    i32.add
    local.tee 2
    i32.const 0
    i32.store8
    local.get 2
    i32.const 1
    i32.add
    local.set 1
    i32.const 4
    local.set 6
    loop  ;; label = @1
      local.get 6
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 7
        i32.add
        local.get 6
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 6
        i32.const 7
        i32.shr_u
        local.set 6
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 7
    i32.add
    local.get 6
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 7
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 109
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 97
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 105
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 110
    i32.store8
    local.get 3
    local.get 0
    i32.const 2
    i32.add
    local.get 2
    i32.add
    i32.add
    local.tee 1
    i32.const 0
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.set 0
    i32.const 7
    local.set 8
    i32.const 0
    local.set 7
    loop  ;; label = @1
      local.get 8
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 7
        i32.add
        local.get 8
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 8
        i32.const 7
        i32.shr_u
        local.set 8
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 7
    i32.add
    local.get 8
    i32.const 127
    i32.and
    i32.store8
    local.get 7
    i32.const 2
    i32.add
    local.get 1
    i32.add
    local.set 1
    i32.const 4
    local.set 6
    i32.const 0
    local.set 5
    loop  ;; label = @1
      local.get 6
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 5
        i32.add
        local.get 6
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        local.get 6
        i32.const 7
        i32.shr_u
        local.set 6
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 5
    i32.add
    local.get 6
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 5
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 99
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 111
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 114
    i32.store8
    local.get 1
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    i32.add
    i32.const 101
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.get 1
    i32.add
    local.tee 0
    i32.const 0
    i32.store8
    local.get 0
    i32.const 0
    i32.store8 offset=1
    local.get 0
    i32.const 2
    i32.add
    local.set 2
    memory.size
    local.tee 1
    local.set 7
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 7
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 7
        i32.const 7
        i32.shr_u
        local.set 7
        br 1 (;@1;)
      end
    end
    local.get 2
    i32.const 5
    i32.store8
    local.get 2
    i32.const 1
    i32.add
    local.set 0
    local.get 6
    i32.const 4
    i32.add
    local.set 5
    i32.const 0
    local.set 8
    loop  ;; label = @1
      local.get 5
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 8
        i32.add
        local.get 5
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 8
        i32.const 1
        i32.add
        local.set 8
        local.get 5
        i32.const 7
        i32.shr_u
        local.set 5
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 8
    i32.add
    local.get 5
    i32.const 127
    i32.and
    i32.store8
    local.get 8
    i32.const 2
    i32.add
    local.get 2
    i32.add
    local.set 0
    i32.const 1
    local.set 5
    i32.const 0
    local.set 7
    loop  ;; label = @1
      local.get 5
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 7
        i32.add
        local.get 5
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 5
        i32.const 7
        i32.shr_u
        local.set 5
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 7
    i32.add
    local.get 5
    i32.const 127
    i32.and
    i32.store8
    local.get 7
    i32.const 1
    i32.add
    local.get 0
    i32.add
    local.tee 3
    i32.const 1
    i32.store8
    local.get 3
    i32.const 1
    i32.add
    local.set 0
    i32.const 0
    local.set 6
    i32.const 0
    local.set 8
    loop  ;; label = @1
      local.get 6
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 8
        i32.add
        local.get 6
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 8
        i32.const 1
        i32.add
        local.set 8
        local.get 6
        i32.const 7
        i32.shr_u
        local.set 6
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 8
    i32.add
    local.get 6
    i32.const 127
    i32.and
    i32.store8
    local.get 3
    local.get 8
    i32.const 2
    i32.add
    local.tee 2
    i32.add
    local.set 0
    i32.const 0
    local.set 8
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 8
        i32.add
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 8
        i32.const 1
        i32.add
        local.set 8
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 8
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    memory.size
    i32.const 16
    i32.shl
    local.tee 4
    local.get 8
    i32.const 1
    i32.add
    local.get 2
    i32.add
    local.get 3
    i32.add
    local.tee 3
    i32.sub
    local.tee 2
    local.set 1
    i32.const 0
    local.set 7
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.const 11
    i32.store8
    local.get 3
    i32.const 1
    i32.add
    local.set 0
    local.get 2
    local.get 7
    i32.sub
    i32.const 2
    i32.sub
    local.set 1
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 6
        i32.add
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 6
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 6
    i32.const 2
    i32.add
    local.get 3
    i32.add
    local.set 0
    i32.const 1
    local.set 1
    i32.const 0
    local.set 7
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 7
        i32.add
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 7
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 7
    i32.const 1
    i32.add
    local.get 0
    i32.add
    local.set 0
    i32.const 0
    local.set 8
    i32.const 0
    local.set 7
    loop  ;; label = @1
      local.get 8
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 7
        i32.add
        local.get 8
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 8
        i32.const 7
        i32.shr_u
        local.set 8
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 7
    i32.add
    local.get 8
    i32.const 127
    i32.and
    i32.store8
    local.get 7
    i32.const 1
    i32.add
    local.get 0
    i32.add
    local.tee 3
    local.set 6
    i32.const 0
    local.set 7
    loop  ;; label = @1
      local.get 6
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 6
        i32.const 7
        i32.shr_u
        local.set 6
        br 1 (;@1;)
      end
    end
    local.get 7
    i32.const 1
    i32.add
    local.set 0
    local.get 4
    local.get 3
    i32.sub
    local.set 1
    i32.const 0
    local.set 7
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.const 65
    i32.store8
    local.get 3
    i32.const 1
    i32.add
    local.set 2
    local.get 0
    local.get 3
    i32.add
    local.get 7
    i32.add
    i32.const 3
    i32.add
    local.set 1
    i32.const 1
    local.set 7
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 7
      if  ;; label = @2
        local.get 1
        i32.const 127
        i32.and
        local.tee 5
        i32.const 64
        i32.and
        local.tee 0
        i32.const 64
        i32.eq
        local.get 1
        i32.const 7
        i32.shr_u
        local.tee 1
        i32.const -1
        i32.eq
        i32.and
        i32.const 1
        i32.const 1
        local.get 0
        local.get 1
        select
        select
        if  ;; label = @3
          i32.const 0
          local.set 7
        else
          local.get 5
          i32.const 128
          i32.or
          local.set 5
        end
        local.get 2
        local.get 6
        i32.add
        local.get 5
        i32.store8
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        br 1 (;@1;)
      end
    end
    local.get 6
    i32.const 1
    i32.add
    local.get 3
    i32.add
    local.tee 0
    i32.const 11
    i32.store8
    local.get 4
    local.get 0
    i32.const 1
    i32.add
    local.tee 2
    i32.sub
    local.tee 0
    local.set 1
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 6
    i32.const 1
    i32.add
    i32.sub
    local.set 5
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 5
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 2
        i32.add
        local.get 5
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 5
        i32.const 7
        i32.shr_u
        local.set 5
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 2
    i32.add
    local.get 5
    i32.const 127
    i32.and
    i32.store8)
  (func $coredump/start_frame (type 4) (param i32 i32)
    (local i32)
    i32.const 0
    i32.load
    i32.const 1836278016
    i32.eq
    if  ;; label = @1
      unreachable
    end
    i32.const 0
    i32.const 0
    i32.load
    i32.const 1
    i32.add
    i32.store
    global.get 0
    local.tee 2
    i32.const 0
    i32.store8
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    local.get 0
    i32.store
    local.get 2
    i32.const 4
    i32.add
    local.tee 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 4
    i32.add
    local.tee 0
    local.get 1
    i32.store
    local.get 0
    i32.const 4
    i32.add
    local.tee 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 4
    i32.add
    global.set 0)
  (func $coredump/add_i32_local (type 5) (param i32)
    (local i32)
    global.get 0
    local.tee 1
    i32.const 127
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    local.get 0
    i32.store
    local.get 1
    i32.const 4
    i32.add
    global.set 0)
  (func $coredump/add_f32_local (type 6) (param f32)
    (local i32)
    global.get 0
    local.tee 1
    i32.const 125
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    local.get 0
    f32.store
    local.get 1
    i32.const 4
    i32.add
    global.set 0)
  (func $coredump/add_f64_local (type 7) (param f64)
    (local i32)
    global.get 0
    local.tee 1
    i32.const 124
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    local.get 0
    f64.store
    local.get 1
    i32.const 8
    i32.add
    global.set 0)
  (func $coredump/add_i64_local (type 8) (param i64)
    (local i32)
    global.get 0
    local.tee 1
    i32.const 126
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    local.get 0
    i64.store
    local.get 1
    i32.const 8
    i32.add
    global.set 0)
  (memory (;0;) 1 1)
  (global (;0;) (mut i32) (i32.const 4))
  (global (;1;) (mut i32) (i32.const 0))
  (global (;2;) (mut i32) (i32.const 0))
  (export "valid" (func $valid))
  (export "invalid" (func $invalid))
  (export "invalid_with_locals" (func $invalid_with_locals))
  (export "invalid_nested2" (func $invalid_nested2))
  (export "_start" (func $invalid_nested2))
  (export "memory" (memory 0)))
