(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32)))
  (type (;5;) (func (param f32)))
  (type (;6;) (func (param f64)))
  (type (;7;) (func (param i64)))
  (func $entry (type 0) (param i32 i32) (result i32)
    global.get $entry_funcidx
    i32.const 2147483647
    i32.eq
    if  ;; label = @1
      i32.const 0
      global.set $entry_funcidx
    end
    local.get 0
    local.get 1
    call $addTwo
    global.get $is_unwinding
    if  ;; label = @1
      i32.const 4
      i32.const 0
      i32.const 2
      i32.const 333
      call $coredump/start_frame
      i32.const 669
      call $coredump/add_i32_local
      i32.const 670
      call $coredump/add_i32_local
      global.get $entry_funcidx
      i32.const 0
      i32.eq
      if  ;; label = @2
        call $coredump/write_coredump
        unreachable
      else
        i32.const 667
        return
      end
    end
    global.get $is_unwinding
    i32.eqz
    if  ;; label = @1
      i32.const 2147483647
      global.set $entry_funcidx
    end)
  (func $addTwo (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
    call $coredump/unreachable_shim
    i32.const 5
    i32.const 1
    i32.const 2
    i32.const 333
    call $coredump/start_frame
    local.get 0
    call $coredump/add_i32_local
    local.get 1
    call $coredump/add_i32_local
    i32.const 666
    return)
  (func $coredump/unreachable_shim (type 1)
    i32.const 1
    global.set $is_unwinding)
  (func $coredump/write_coredump (type 2)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    i32.load
    i32.const 1836278016
    i32.eq
    if  ;; label = @1
      unreachable
    end
    global.get $frames_ptr_global
    local.set 5
    global.get $frames_count_global
    local.set 0
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 3
    local.get 5
    i32.add
    i32.const 17
    i32.add
    local.tee 2
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 0
    i32.const 1
    i32.add
    local.set 6
    global.get $frames_count_global
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 6
    i32.add
    i32.const 26
    i32.add
    i32.const 0
    local.get 5
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
    local.get 2
    local.set 0
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=9
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 0
    i32.const 127
    i32.and
    i32.store8 offset=9
    local.get 1
    i32.const 10
    i32.add
    local.set 2
    i32.const 9
    local.set 1
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 2
        local.get 4
        i32.add
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 2
    local.get 4
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 4
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 99
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 111
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 114
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 101
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 115
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 116
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 97
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 99
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 1
    local.get 2
    i32.add
    i32.const 107
    i32.store8
    i32.const 0
    local.set 0
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    i32.add
    local.tee 1
    i32.const 0
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.set 3
    i32.const 4
    local.set 2
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 3
        i32.add
        local.get 2
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 3
    i32.add
    local.get 2
    i32.const 127
    i32.and
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 3
    i32.add
    i32.const 109
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 3
    i32.add
    i32.const 97
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 3
    i32.add
    i32.const 105
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 3
    i32.add
    i32.const 110
    i32.store8
    local.get 1
    local.get 0
    i32.const 2
    i32.add
    i32.add
    local.set 1
    global.get $frames_count_global
    local.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 3
    i32.add
    local.get 0
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 3
    i32.const 1
    i32.add
    i32.add
    local.get 5
    i32.add
    local.tee 0
    i32.const 0
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.set 1
    i32.const 7
    local.set 4
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 4
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.get 4
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 4
        i32.const 7
        i32.shr_u
        local.set 4
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 3
    i32.add
    local.get 4
    i32.const 127
    i32.and
    i32.store8
    local.get 0
    local.get 3
    i32.const 2
    i32.add
    i32.add
    local.set 2
    i32.const 4
    local.set 1
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 2
        i32.add
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 2
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 99
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 111
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 114
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.tee 0
    local.get 2
    i32.add
    i32.const 101
    i32.store8
    local.get 2
    local.get 0
    i32.const 1
    i32.add
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
    local.set 5
    memory.size
    local.tee 2
    local.set 3
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 5
    i32.const 5
    i32.store8
    local.get 5
    i32.const 1
    i32.add
    local.set 3
    local.get 1
    i32.const 4
    i32.add
    local.set 0
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        local.get 4
        i32.add
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 3
    local.get 4
    i32.add
    local.get 0
    i32.const 127
    i32.and
    i32.store8
    local.get 5
    local.get 4
    i32.const 2
    i32.add
    i32.add
    local.set 1
    i32.const 1
    local.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 3
    i32.add
    local.get 0
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 3
    i32.const 1
    i32.add
    i32.add
    local.tee 0
    i32.const 1
    i32.store8
    local.get 0
    i32.const 1
    i32.add
    local.set 3
    i32.const 0
    local.set 1
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        local.get 4
        i32.add
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 3
    local.get 4
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 4
    i32.const 2
    i32.add
    local.tee 3
    local.get 0
    i32.add
    local.set 5
    local.get 2
    local.set 1
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 4
        local.get 5
        i32.add
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 4
    local.get 5
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    memory.size
    i32.const 16
    i32.shl
    local.tee 5
    local.get 0
    local.get 3
    local.get 4
    i32.const 1
    i32.add
    i32.add
    i32.add
    local.tee 1
    i32.sub
    local.tee 0
    local.set 2
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 1
    i32.const 11
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.set 2
    local.get 0
    local.get 3
    i32.sub
    i32.const 2
    i32.sub
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 2
        i32.add
        local.get 3
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 2
    i32.add
    local.get 3
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 0
    i32.const 2
    i32.add
    i32.add
    local.set 0
    i32.const 1
    local.set 2
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 3
        i32.add
        local.get 2
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 3
    i32.add
    local.get 2
    i32.const 127
    i32.and
    i32.store8
    local.get 0
    local.get 3
    i32.const 1
    i32.add
    i32.add
    local.set 0
    i32.const 0
    local.set 4
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 4
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 3
        i32.add
        local.get 4
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 4
        i32.const 7
        i32.shr_u
        local.set 4
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 3
    i32.add
    local.get 4
    i32.const 127
    i32.and
    i32.store8
    local.get 0
    local.get 3
    i32.const 1
    i32.add
    i32.add
    local.tee 6
    local.set 1
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.const 1
    i32.add
    local.set 0
    local.get 5
    local.get 6
    i32.sub
    local.set 2
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 6
    i32.const 65
    i32.store8
    local.get 6
    i32.const 1
    i32.add
    local.set 4
    local.get 3
    local.get 0
    local.get 6
    i32.add
    i32.add
    i32.const 3
    i32.add
    local.set 2
    i32.const 1
    local.set 3
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 3
      if  ;; label = @2
        local.get 2
        i32.const 127
        i32.and
        local.tee 0
        i32.const 64
        i32.and
        local.tee 7
        i32.const 64
        i32.eq
        local.get 2
        i32.const 7
        i32.shr_u
        local.tee 2
        i32.const -1
        i32.eq
        i32.and
        i32.const 1
        i32.const 1
        local.get 7
        local.get 2
        select
        select
        if  ;; label = @3
          i32.const 0
          local.set 3
        else
          local.get 0
          i32.const 128
          i32.or
          local.set 0
        end
        local.get 1
        local.get 4
        i32.add
        local.get 0
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 6
    local.get 1
    i32.const 1
    i32.add
    i32.add
    local.tee 0
    i32.const 11
    i32.store8
    local.get 5
    local.get 0
    i32.const 1
    i32.add
    local.tee 1
    i32.sub
    local.tee 2
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 2
    local.get 0
    i32.const 1
    i32.add
    i32.sub
    local.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 3
    i32.add
    local.get 0
    i32.const 127
    i32.and
    i32.store8)
  (func $coredump/start_frame (type 3) (param i32 i32 i32 i32)
    (local i32 i32)
    i32.const 0
    i32.load
    i32.const 1836278016
    i32.eq
    if  ;; label = @1
      unreachable
    end
    global.get $frames_ptr_global
    local.tee 5
    i32.const 0
    i32.store8
    local.get 5
    i32.const 1
    i32.add
    local.set 5
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 4
        local.get 5
        i32.add
        local.get 3
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 4
    local.get 5
    i32.add
    local.get 3
    i32.const 127
    i32.and
    i32.store8
    local.get 5
    local.get 4
    i32.const 1
    i32.add
    i32.add
    local.set 3
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        local.get 4
        i32.add
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 3
    local.get 4
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 3
    local.get 4
    i32.const 1
    i32.add
    i32.add
    local.set 1
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 4
        i32.add
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 4
    i32.add
    local.get 0
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 4
    i32.const 1
    i32.add
    i32.add
    local.set 0
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 4
        i32.add
        local.get 2
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 4
    i32.add
    local.get 2
    i32.const 127
    i32.and
    i32.store8
    local.get 0
    local.get 4
    i32.const 1
    i32.add
    i32.add
    local.set 1
    i32.const 0
    local.set 0
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 4
        i32.add
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 4
    i32.add
    local.get 0
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 4
    i32.const 1
    i32.add
    i32.add
    global.set $frames_ptr_global
    global.get $frames_count_global
    i32.const 1
    i32.add
    global.set $frames_count_global)
  (func $coredump/add_i32_local (type 4) (param i32)
    (local i32)
    global.get $frames_ptr_global
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
    global.set $frames_ptr_global)
  (func $coredump/add_f32_local (type 5) (param f32)
    (local i32)
    global.get $frames_ptr_global
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
    global.set $frames_ptr_global)
  (func $coredump/add_f64_local (type 6) (param f64)
    (local i32)
    global.get $frames_ptr_global
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
    global.set $frames_ptr_global)
  (func $coredump/add_i64_local (type 7) (param i64)
    (local i32)
    global.get $frames_ptr_global
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
    global.set $frames_ptr_global)
  (memory (;0;) 10)
  (global $one i32 (i32.const 1))
  (global $second i64 (i64.const 2))
  (global $frames_ptr_global (mut i32) (i32.const 0))
  (global $frames_count_global (mut i32) (i32.const 0))
  (global $is_unwinding (mut i32) (i32.const 0))
  (global $entry_funcidx (mut i32) (i32.const 2147483647))
  (export "addTwo" (func $entry))
  (export "memory" (memory 0)))
