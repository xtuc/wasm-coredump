(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32)))
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
      i32.const 0
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
  (func (;3;) (type 2) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 4
    i32.load
    local.tee 4
    i32.const 16
    i32.add
    local.tee 2
    local.set 1
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
    i32.const 26
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
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 5
        local.get 2
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=9
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 5
    local.get 2
    i32.const 127
    i32.and
    i32.store8 offset=9
    local.get 5
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
    local.get 2
    local.get 6
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 2
    local.get 6
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 99
    i32.store8
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 111
    i32.store8
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 114
    i32.store8
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 101
    i32.store8
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 115
    i32.store8
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 116
    i32.store8
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 97
    i32.store8
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 99
    i32.store8
    local.get 2
    local.get 1
    i32.const 1
    i32.add
    local.tee 1
    i32.add
    i32.const 107
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.get 2
    i32.add
    local.tee 1
    i32.const 0
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.set 2
    i32.const 4
    local.set 5
    loop  ;; label = @1
      local.get 5
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 2
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
    local.get 2
    local.get 7
    i32.add
    local.get 5
    i32.const 127
    i32.and
    i32.store8
    local.get 2
    local.get 7
    i32.const 1
    i32.add
    local.tee 3
    i32.add
    i32.const 109
    i32.store8
    local.get 2
    local.get 3
    i32.const 1
    i32.add
    local.tee 3
    i32.add
    i32.const 97
    i32.store8
    local.get 2
    local.get 3
    i32.const 1
    i32.add
    local.tee 3
    i32.add
    i32.const 105
    i32.store8
    local.get 2
    local.get 3
    i32.const 1
    i32.add
    local.tee 3
    i32.add
    i32.const 110
    i32.store8
    local.get 4
    local.get 3
    i32.const 2
    i32.add
    local.get 1
    i32.add
    i32.add
    local.tee 2
    i32.const 0
    i32.store8
    local.get 2
    i32.const 1
    i32.add
    local.set 4
    i32.const 7
    local.set 1
    i32.const 0
    local.set 3
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
    local.get 4
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 3
    i32.const 2
    i32.add
    local.get 2
    i32.add
    local.set 1
    i32.const 4
    local.set 5
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 5
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.get 5
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 5
        i32.const 7
        i32.shr_u
        local.set 5
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 3
    i32.add
    local.get 5
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    local.get 3
    i32.const 1
    i32.add
    local.tee 2
    i32.add
    i32.const 99
    i32.store8
    local.get 1
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    i32.add
    i32.const 111
    i32.store8
    local.get 1
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    i32.add
    i32.const 114
    i32.store8
    local.get 1
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    i32.add
    i32.const 101
    i32.store8
    local.get 2
    i32.const 1
    i32.add
    local.get 1
    i32.add
    local.tee 1
    i32.const 0
    i32.store8
    local.get 1
    i32.const 0
    i32.store8 offset=1
    local.get 1
    i32.const 2
    i32.add
    local.set 3
    memory.size
    local.tee 2
    local.set 7
    i32.const 0
    local.set 5
    loop  ;; label = @1
      local.get 7
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        local.get 7
        i32.const 7
        i32.shr_u
        local.set 7
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.const 5
    i32.store8
    local.get 3
    i32.const 1
    i32.add
    local.set 4
    local.get 5
    i32.const 4
    i32.add
    local.set 6
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 6
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 4
        i32.add
        local.get 6
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 6
        i32.const 7
        i32.shr_u
        local.set 6
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 4
    i32.add
    local.get 6
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    i32.const 2
    i32.add
    local.get 3
    i32.add
    local.set 1
    i32.const 1
    local.set 3
    i32.const 0
    local.set 7
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 7
        i32.add
        local.get 3
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 7
    i32.add
    local.get 3
    i32.const 127
    i32.and
    i32.store8
    local.get 7
    i32.const 1
    i32.add
    local.get 1
    i32.add
    local.tee 3
    i32.const 1
    i32.store8
    local.get 3
    i32.const 1
    i32.add
    local.set 4
    i32.const 0
    local.set 5
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 5
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 4
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
    local.get 4
    i32.add
    local.get 5
    i32.const 127
    i32.and
    i32.store8
    local.get 3
    local.get 1
    i32.const 2
    i32.add
    local.tee 4
    i32.add
    local.set 5
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 5
        i32.add
        local.get 2
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 5
    i32.add
    local.get 2
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.get 4
    i32.add
    local.get 3
    i32.add
    local.set 2
    local.get 0
    local.set 1
    i32.const 0
    local.set 5
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        local.get 1
        i32.const 7
        i32.shr_u
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 2
    i32.const 6
    i32.store8
    local.get 2
    i32.const 1
    i32.add
    local.set 3
    local.get 5
    i32.const 1
    i32.add
    local.get 0
    i32.const 5
    i32.mul
    i32.add
    local.set 4
    i32.const 0
    local.set 1
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
        local.get 1
        i32.const 1
        i32.add
        local.set 1
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
    local.get 1
    i32.const 2
    i32.add
    local.get 2
    i32.add
    local.set 3
    local.get 0
    local.set 2
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.get 2
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 3
    i32.add
    local.get 2
    i32.const 127
    i32.and
    i32.store8
    local.get 1
    i32.const 1
    i32.add
    local.get 3
    i32.add
    local.set 2
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 0
      local.get 1
      i32.gt_u
      if  ;; label = @2
        local.get 2
        i32.const 127
        i32.store8
        i32.const 0
        local.set 5
        local.get 2
        i32.const 0
        i32.store8 offset=1
        local.get 2
        i32.const 2
        i32.add
        local.tee 3
        i32.const 65
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 8
        local.get 1
        i32.const 9
        i32.add
        local.set 4
        i32.const 1
        local.set 3
        loop  ;; label = @3
          local.get 3
          if  ;; label = @4
            local.get 4
            i32.const 127
            i32.and
            local.tee 7
            i32.const 64
            i32.and
            local.tee 6
            i32.const 64
            i32.eq
            local.get 4
            i32.const 7
            i32.shr_u
            local.tee 4
            i32.const -1
            i32.eq
            i32.and
            i32.const 1
            i32.const 1
            local.get 6
            local.get 4
            select
            select
            if  ;; label = @5
              i32.const 0
              local.set 3
            else
              local.get 7
              i32.const 128
              i32.or
              local.set 7
            end
            local.get 5
            local.get 8
            i32.add
            local.get 7
            i32.store8
            local.get 5
            i32.const 1
            i32.add
            local.set 5
            br 1 (;@3;)
          end
        end
        local.get 2
        local.get 5
        i32.const 3
        i32.add
        local.tee 3
        i32.add
        i32.const 11
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.get 2
        i32.add
        local.set 2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        br 1 (;@1;)
      end
    end
    memory.size
    i32.const 16
    i32.shl
    local.tee 0
    local.get 2
    i32.sub
    local.tee 1
    local.set 4
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 4
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 4
        i32.const 7
        i32.shr_u
        local.set 4
        br 1 (;@1;)
      end
    end
    local.get 2
    i32.const 11
    i32.store8
    local.get 2
    i32.const 1
    i32.add
    local.set 3
    local.get 1
    local.get 6
    i32.sub
    i32.const 2
    i32.sub
    local.set 5
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 5
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 3
        local.get 4
        i32.add
        local.get 5
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 5
        i32.const 7
        i32.shr_u
        local.set 5
        br 1 (;@1;)
      end
    end
    local.get 3
    local.get 4
    i32.add
    local.get 5
    i32.const 127
    i32.and
    i32.store8
    local.get 4
    i32.const 2
    i32.add
    local.get 2
    i32.add
    local.set 2
    i32.const 1
    local.set 1
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 2
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
    local.get 2
    local.get 6
    i32.add
    local.get 1
    i32.const 127
    i32.and
    i32.store8
    local.get 6
    i32.const 1
    i32.add
    local.get 2
    i32.add
    local.set 1
    i32.const 0
    local.set 3
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 6
        i32.add
        local.get 3
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 6
    i32.add
    local.get 3
    i32.const 127
    i32.and
    i32.store8
    local.get 6
    i32.const 1
    i32.add
    local.get 1
    i32.add
    local.tee 2
    local.set 3
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 3
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 3
        i32.const 7
        i32.shr_u
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 6
    i32.const 1
    i32.add
    local.set 1
    local.get 0
    local.get 2
    i32.sub
    local.set 4
    i32.const 0
    local.set 6
    loop  ;; label = @1
      local.get 4
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 4
        i32.const 7
        i32.shr_u
        local.set 4
        br 1 (;@1;)
      end
    end
    local.get 2
    i32.const 65
    i32.store8
    local.get 2
    i32.const 1
    i32.add
    local.set 7
    local.get 1
    local.get 2
    i32.add
    local.get 6
    i32.add
    i32.const 3
    i32.add
    local.set 1
    i32.const 1
    local.set 5
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 5
      if  ;; label = @2
        local.get 1
        i32.const 127
        i32.and
        local.tee 4
        i32.const 64
        i32.and
        local.tee 6
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
        local.get 6
        local.get 1
        select
        select
        if  ;; label = @3
          i32.const 0
          local.set 5
        else
          local.get 4
          i32.const 128
          i32.or
          local.set 4
        end
        local.get 3
        local.get 7
        i32.add
        local.get 4
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.const 1
    i32.add
    local.get 2
    i32.add
    local.tee 1
    i32.const 11
    i32.store8
    local.get 0
    local.get 1
    i32.const 1
    i32.add
    local.tee 0
    i32.sub
    local.tee 2
    local.set 1
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
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
    i32.const 1
    i32.add
    i32.sub
    local.set 2
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 2
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 1
        i32.add
        local.get 2
        i32.const 127
        i32.and
        i32.const 128
        i32.or
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const 7
        i32.shr_u
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 1
    i32.add
    local.get 2
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
