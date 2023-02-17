
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

export function set_frame1(code_offset: u32, arg0Type: u8, arg0: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 1

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 13, 0, 1)
}

export function set_frame2(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 2

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 18, 0, 1)
}

export function set_frame3(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 3

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 23, 0, 1)
}

export function set_frame4(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 4

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 28, 0, 1)
}

export function set_frame5(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 5

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 33, 0, 1)
}

export function set_frame6(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 6

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 38, 0, 1)
}

export function set_frame7(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 7

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 43, 0, 1)
}

export function set_frame8(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 8

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 48, 0, 1)
}

export function set_frame9(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 9

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 53, 0, 1)
}

export function set_frame10(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 10

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 58, 0, 1)
}

export function set_frame11(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 11

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 63, 0, 1)
}

export function set_frame12(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 12

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 68, 0, 1)
}

export function set_frame13(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 13

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 73, 0, 1)
}

export function set_frame14(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 14

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 78, 0, 1)
}

export function set_frame15(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 15

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 83, 0, 1)
}

export function set_frame16(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 16

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 88, 0, 1)
}

export function set_frame17(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 17

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 93, 0, 1)
}

export function set_frame18(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 18

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 98, 0, 1)
}

export function set_frame19(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 19

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 103, 0, 1)
}

export function set_frame20(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 20

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 108, 0, 1)
}

export function set_frame21(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 21

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 113, 0, 1)
}

export function set_frame22(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 22

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 118, 0, 1)
}

export function set_frame23(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32, arg22Type: u8, arg22: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 23

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)
store<u8>(base, arg22Type, 118, 1)
store<i32>(base, arg22, 119, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 123, 0, 1)
}

export function set_frame24(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32, arg22Type: u8, arg22: i32, arg23Type: u8, arg23: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 24

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)
store<u8>(base, arg22Type, 118, 1)
store<i32>(base, arg22, 119, 1)
store<u8>(base, arg23Type, 123, 1)
store<i32>(base, arg23, 124, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 128, 0, 1)
}

export function set_frame25(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32, arg22Type: u8, arg22: i32, arg23Type: u8, arg23: i32, arg24Type: u8, arg24: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 25

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)
store<u8>(base, arg22Type, 118, 1)
store<i32>(base, arg22, 119, 1)
store<u8>(base, arg23Type, 123, 1)
store<i32>(base, arg23, 124, 1)
store<u8>(base, arg24Type, 128, 1)
store<i32>(base, arg24, 129, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 133, 0, 1)
}

export function set_frame26(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32, arg22Type: u8, arg22: i32, arg23Type: u8, arg23: i32, arg24Type: u8, arg24: i32, arg25Type: u8, arg25: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 26

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)
store<u8>(base, arg22Type, 118, 1)
store<i32>(base, arg22, 119, 1)
store<u8>(base, arg23Type, 123, 1)
store<i32>(base, arg23, 124, 1)
store<u8>(base, arg24Type, 128, 1)
store<i32>(base, arg24, 129, 1)
store<u8>(base, arg25Type, 133, 1)
store<i32>(base, arg25, 134, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 138, 0, 1)
}

export function set_frame27(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32, arg22Type: u8, arg22: i32, arg23Type: u8, arg23: i32, arg24Type: u8, arg24: i32, arg25Type: u8, arg25: i32, arg26Type: u8, arg26: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 27

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)
store<u8>(base, arg22Type, 118, 1)
store<i32>(base, arg22, 119, 1)
store<u8>(base, arg23Type, 123, 1)
store<i32>(base, arg23, 124, 1)
store<u8>(base, arg24Type, 128, 1)
store<i32>(base, arg24, 129, 1)
store<u8>(base, arg25Type, 133, 1)
store<i32>(base, arg25, 134, 1)
store<u8>(base, arg26Type, 138, 1)
store<i32>(base, arg26, 139, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 143, 0, 1)
}

export function set_frame28(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32, arg22Type: u8, arg22: i32, arg23Type: u8, arg23: i32, arg24Type: u8, arg24: i32, arg25Type: u8, arg25: i32, arg26Type: u8, arg26: i32, arg27Type: u8, arg27: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 28

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)
store<u8>(base, arg22Type, 118, 1)
store<i32>(base, arg22, 119, 1)
store<u8>(base, arg23Type, 123, 1)
store<i32>(base, arg23, 124, 1)
store<u8>(base, arg24Type, 128, 1)
store<i32>(base, arg24, 129, 1)
store<u8>(base, arg25Type, 133, 1)
store<i32>(base, arg25, 134, 1)
store<u8>(base, arg26Type, 138, 1)
store<i32>(base, arg26, 139, 1)
store<u8>(base, arg27Type, 143, 1)
store<i32>(base, arg27, 144, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 148, 0, 1)
}

export function set_frame29(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32, arg22Type: u8, arg22: i32, arg23Type: u8, arg23: i32, arg24Type: u8, arg24: i32, arg25Type: u8, arg25: i32, arg26Type: u8, arg26: i32, arg27Type: u8, arg27: i32, arg28Type: u8, arg28: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 29

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)
store<u8>(base, arg22Type, 118, 1)
store<i32>(base, arg22, 119, 1)
store<u8>(base, arg23Type, 123, 1)
store<i32>(base, arg23, 124, 1)
store<u8>(base, arg24Type, 128, 1)
store<i32>(base, arg24, 129, 1)
store<u8>(base, arg25Type, 133, 1)
store<i32>(base, arg25, 134, 1)
store<u8>(base, arg26Type, 138, 1)
store<i32>(base, arg26, 139, 1)
store<u8>(base, arg27Type, 143, 1)
store<i32>(base, arg27, 144, 1)
store<u8>(base, arg28Type, 148, 1)
store<i32>(base, arg28, 149, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 153, 0, 1)
}

export function set_frame30(code_offset: u32, arg0Type: u8, arg0: i32, arg1Type: u8, arg1: i32, arg2Type: u8, arg2: i32, arg3Type: u8, arg3: i32, arg4Type: u8, arg4: i32, arg5Type: u8, arg5: i32, arg6Type: u8, arg6: i32, arg7Type: u8, arg7: i32, arg8Type: u8, arg8: i32, arg9Type: u8, arg9: i32, arg10Type: u8, arg10: i32, arg11Type: u8, arg11: i32, arg12Type: u8, arg12: i32, arg13Type: u8, arg13: i32, arg14Type: u8, arg14: i32, arg15Type: u8, arg15: i32, arg16Type: u8, arg16: i32, arg17Type: u8, arg17: i32, arg18Type: u8, arg18: i32, arg19Type: u8, arg19: i32, arg20Type: u8, arg20: i32, arg21Type: u8, arg21: i32, arg22Type: u8, arg22: i32, arg23Type: u8, arg23: i32, arg24Type: u8, arg24: i32, arg25Type: u8, arg25: i32, arg26Type: u8, arg26: i32, arg27Type: u8, arg27: i32, arg28Type: u8, arg28: i32, arg29Type: u8, arg29: i32): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = 30

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
store<u8>(base, arg0Type, 8, 1)
store<i32>(base, arg0, 9, 1)
store<u8>(base, arg1Type, 13, 1)
store<i32>(base, arg1, 14, 1)
store<u8>(base, arg2Type, 18, 1)
store<i32>(base, arg2, 19, 1)
store<u8>(base, arg3Type, 23, 1)
store<i32>(base, arg3, 24, 1)
store<u8>(base, arg4Type, 28, 1)
store<i32>(base, arg4, 29, 1)
store<u8>(base, arg5Type, 33, 1)
store<i32>(base, arg5, 34, 1)
store<u8>(base, arg6Type, 38, 1)
store<i32>(base, arg6, 39, 1)
store<u8>(base, arg7Type, 43, 1)
store<i32>(base, arg7, 44, 1)
store<u8>(base, arg8Type, 48, 1)
store<i32>(base, arg8, 49, 1)
store<u8>(base, arg9Type, 53, 1)
store<i32>(base, arg9, 54, 1)
store<u8>(base, arg10Type, 58, 1)
store<i32>(base, arg10, 59, 1)
store<u8>(base, arg11Type, 63, 1)
store<i32>(base, arg11, 64, 1)
store<u8>(base, arg12Type, 68, 1)
store<i32>(base, arg12, 69, 1)
store<u8>(base, arg13Type, 73, 1)
store<i32>(base, arg13, 74, 1)
store<u8>(base, arg14Type, 78, 1)
store<i32>(base, arg14, 79, 1)
store<u8>(base, arg15Type, 83, 1)
store<i32>(base, arg15, 84, 1)
store<u8>(base, arg16Type, 88, 1)
store<i32>(base, arg16, 89, 1)
store<u8>(base, arg17Type, 93, 1)
store<i32>(base, arg17, 94, 1)
store<u8>(base, arg18Type, 98, 1)
store<i32>(base, arg18, 99, 1)
store<u8>(base, arg19Type, 103, 1)
store<i32>(base, arg19, 104, 1)
store<u8>(base, arg20Type, 108, 1)
store<i32>(base, arg20, 109, 1)
store<u8>(base, arg21Type, 113, 1)
store<i32>(base, arg21, 114, 1)
store<u8>(base, arg22Type, 118, 1)
store<i32>(base, arg22, 119, 1)
store<u8>(base, arg23Type, 123, 1)
store<i32>(base, arg23, 124, 1)
store<u8>(base, arg24Type, 128, 1)
store<i32>(base, arg24, 129, 1)
store<u8>(base, arg25Type, 133, 1)
store<i32>(base, arg25, 134, 1)
store<u8>(base, arg26Type, 138, 1)
store<i32>(base, arg26, 139, 1)
store<u8>(base, arg27Type, 143, 1)
store<i32>(base, arg27, 144, 1)
store<u8>(base, arg28Type, 148, 1)
store<i32>(base, arg28, 149, 1)
store<u8>(base, arg29Type, 153, 1)
store<i32>(base, arg29, 154, 1)

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + 158, 0, 1)
}

