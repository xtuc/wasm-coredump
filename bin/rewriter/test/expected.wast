(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func))
  (type (;3;) (func (param i32 i32 i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 1
    global.get 0
    if  ;; label = @1
      i32.const 0
      i32.const 669
      i32.const 670
      call 4
      call 3
      unreachable
    end)
  (func (;1;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
    call 2
    i32.const 1
    i32.const 669
    i32.const 670
    call 4
    i32.const 666
    return)
  (func (;2;) (type 1)
    i32.const 1
    global.set 0)
  (func (;3;) (type 2)
    (local i32 i32 i32 i32 i32)
    i32.const 4
    i32.load
    local.tee 4
    local.set 0
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
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
    i32.const 20
    i32.add
    i32.const 0
    local.get 4
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
    local.get 4
    i32.const 10
    i32.add
    local.set 1
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        local.get 1
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=9
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
    local.get 1
    i32.const 127
    i32.and
    i32.store8 offset=9
    local.get 3
    i32.const 10
    i32.add
    local.set 1
    i32.const 9
    local.set 0
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 2
        i32.add
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 2
    i32.add
    local.get 0
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 2
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
    memory.size
    i32.const 16
    i32.shl
    local.tee 3
    local.get 4
    local.get 0
    i32.const 1
    i32.add
    local.get 1
    i32.add
    i32.add
    local.tee 4
    i32.sub
    local.tee 2
    local.set 1
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
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
    local.get 4
    i32.const 11
    i32.store8
    local.get 4
    i32.const 1
    i32.add
    local.set 1
    local.get 2
    local.get 0
    i32.sub
    i32.const 2
    i32.sub
    local.set 0
    i32.const 0
    local.set 2
    loop  ;; label = @1
      local.get 0
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 2
        i32.add
        local.get 0
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 0
        i32.const 7
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 2
    i32.add
    local.get 0
    i32.const 127
    i32.and
    i32.store8
    local.get 2
    i32.const 2
    i32.add
    local.get 4
    i32.add
    local.set 2
    i32.const 1
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
    local.get 2
    i32.add
    local.set 2
    i32.const 1
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
    local.get 3
    local.get 0
    i32.const 1
    i32.add
    local.get 2
    i32.add
    local.tee 2
    i32.sub
    local.tee 3
    local.set 1
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
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
    local.get 3
    local.get 0
    i32.const 1
    i32.add
    i32.sub
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
    i32.store8)
  (func (;4;) (type 3) (param i32 i32 i32)
    (local i32)
    i32.const 4
    i32.load align=1
    local.tee 3
    i32.eqz
    if  ;; label = @1
      i32.const 8
      local.set 3
    end
    local.get 3
    local.get 0
    i32.store align=1
    local.get 3
    i32.const 2
    i32.store offset=4 align=1
    local.get 3
    local.get 1
    i32.store offset=8 align=1
    local.get 3
    local.get 2
    i32.store offset=12 align=1
    i32.const 0
    i32.const 0
    i32.load align=1
    i32.const 1
    i32.add
    i32.store align=1
    i32.const 4
    local.get 3
    i32.const 16
    i32.add
    i32.store align=1)
  (memory (;0;) 10)
  (global (;0;) (mut i32) (i32.const 0))
  (export "addTwo" (func 0))
  (export "memory" (memory 0)))
