
@inline
function get_next_frame_ptr(): u32 {
  const next_frame = load<u32>(4, 0, 1)
  let ptr: u32 = next_frame
  if (ptr === 0) {
    // It's the first frame we add, it starts after the "number of frames (u32)"
    // and "next frame offset (u32)".
    ptr = sizeof<u32>() * 2
  }

  return ptr
}

export function set_frame0(code_offset: u32, ): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 0

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals


  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 8, 0, 1)
}

export function set_frame1(code_offset: u32, arg0: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 1

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 12, 0, 1)
}

export function set_frame2(code_offset: u32, arg0: u32, arg1: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 2

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 16, 0, 1)
}

export function set_frame3(code_offset: u32, arg0: u32, arg1: u32, arg2: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 3

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 20, 0, 1)
}

export function set_frame4(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 4

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 24, 0, 1)
}

export function set_frame5(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 5

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 28, 0, 1)
}

export function set_frame6(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 6

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 32, 0, 1)
}

export function set_frame7(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 7

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 36, 0, 1)
}

export function set_frame8(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 8

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 40, 0, 1)
}

export function set_frame9(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 9

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 44, 0, 1)
}

export function set_frame10(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 10

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 48, 0, 1)
}

export function set_frame11(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 11

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 52, 0, 1)
}

export function set_frame12(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 12

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 56, 0, 1)
}

export function set_frame13(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 13

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 60, 0, 1)
}

export function set_frame14(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 14

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 64, 0, 1)
}

export function set_frame15(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 15

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 68, 0, 1)
}

export function set_frame16(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 16

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 72, 0, 1)
}

export function set_frame17(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 17

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 76, 0, 1)
}

export function set_frame18(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 18

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 80, 0, 1)
}

export function set_frame19(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 19

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 84, 0, 1)
}

export function set_frame20(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 20

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 88, 0, 1)
}

export function set_frame21(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 21

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 92, 0, 1)
}

export function set_frame22(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 22

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 96, 0, 1)
}

export function set_frame23(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32, arg22: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 23

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)
store<u32>(base, arg22, 96, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 100, 0, 1)
}

export function set_frame24(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32, arg22: u32, arg23: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 24

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)
store<u32>(base, arg22, 96, 1)
store<u32>(base, arg23, 100, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 104, 0, 1)
}

export function set_frame25(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32, arg22: u32, arg23: u32, arg24: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 25

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)
store<u32>(base, arg22, 96, 1)
store<u32>(base, arg23, 100, 1)
store<u32>(base, arg24, 104, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 108, 0, 1)
}

export function set_frame26(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32, arg22: u32, arg23: u32, arg24: u32, arg25: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 26

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)
store<u32>(base, arg22, 96, 1)
store<u32>(base, arg23, 100, 1)
store<u32>(base, arg24, 104, 1)
store<u32>(base, arg25, 108, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 112, 0, 1)
}

export function set_frame27(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32, arg22: u32, arg23: u32, arg24: u32, arg25: u32, arg26: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 27

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)
store<u32>(base, arg22, 96, 1)
store<u32>(base, arg23, 100, 1)
store<u32>(base, arg24, 104, 1)
store<u32>(base, arg25, 108, 1)
store<u32>(base, arg26, 112, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 116, 0, 1)
}

export function set_frame28(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32, arg22: u32, arg23: u32, arg24: u32, arg25: u32, arg26: u32, arg27: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 28

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)
store<u32>(base, arg22, 96, 1)
store<u32>(base, arg23, 100, 1)
store<u32>(base, arg24, 104, 1)
store<u32>(base, arg25, 108, 1)
store<u32>(base, arg26, 112, 1)
store<u32>(base, arg27, 116, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 120, 0, 1)
}

export function set_frame29(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32, arg22: u32, arg23: u32, arg24: u32, arg25: u32, arg26: u32, arg27: u32, arg28: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 29

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)
store<u32>(base, arg22, 96, 1)
store<u32>(base, arg23, 100, 1)
store<u32>(base, arg24, 104, 1)
store<u32>(base, arg25, 108, 1)
store<u32>(base, arg26, 112, 1)
store<u32>(base, arg27, 116, 1)
store<u32>(base, arg28, 120, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 124, 0, 1)
}

export function set_frame30(code_offset: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32, arg8: u32, arg9: u32, arg10: u32, arg11: u32, arg12: u32, arg13: u32, arg14: u32, arg15: u32, arg16: u32, arg17: u32, arg18: u32, arg19: u32, arg20: u32, arg21: u32, arg22: u32, arg23: u32, arg24: u32, arg25: u32, arg26: u32, arg27: u32, arg28: u32, arg29: u32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 30

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u32>(base, arg0, 8, 1)
store<u32>(base, arg1, 12, 1)
store<u32>(base, arg2, 16, 1)
store<u32>(base, arg3, 20, 1)
store<u32>(base, arg4, 24, 1)
store<u32>(base, arg5, 28, 1)
store<u32>(base, arg6, 32, 1)
store<u32>(base, arg7, 36, 1)
store<u32>(base, arg8, 40, 1)
store<u32>(base, arg9, 44, 1)
store<u32>(base, arg10, 48, 1)
store<u32>(base, arg11, 52, 1)
store<u32>(base, arg12, 56, 1)
store<u32>(base, arg13, 60, 1)
store<u32>(base, arg14, 64, 1)
store<u32>(base, arg15, 68, 1)
store<u32>(base, arg16, 72, 1)
store<u32>(base, arg17, 76, 1)
store<u32>(base, arg18, 80, 1)
store<u32>(base, arg19, 84, 1)
store<u32>(base, arg20, 88, 1)
store<u32>(base, arg21, 92, 1)
store<u32>(base, arg22, 96, 1)
store<u32>(base, arg23, 100, 1)
store<u32>(base, arg24, 104, 1)
store<u32>(base, arg25, 108, 1)
store<u32>(base, arg26, 112, 1)
store<u32>(base, arg27, 116, 1)
store<u32>(base, arg28, 120, 1)
store<u32>(base, arg29, 124, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 128, 0, 1)
}

