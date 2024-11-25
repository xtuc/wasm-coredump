import * as wasm from 'asc-wasm/assembly'

// Pointer or cursor to the latest frame
// Assumed to be globalidx 0 by bin/rewriter/src/runtime.rs
var frames_ptr: u32 = 0;

// Keep track of number of frames
// Assumed to be globalidx 1 by bin/rewriter/src/runtime.rs
var frame_count: u32 = 0;

@inline
function write_u8(ptr: u32, v: u8): u32 {
  store<u8>(ptr, v)
  return 1;
}

@inline
function write_process_info(ptr: u32): u32 {
  let wrote: u32 = 0;
  wrote += write_u8(ptr, 0); // type
  wrote += write_u8(ptr + wrote, 0); // name length
  return wrote
}

@inline
function write_thread_info(ptr: u32): u32 {
  let wrote: u32 = 0;
  wrote += write_u8(ptr, 0x0)
  // Thead name: main.
  wrote += wasm.write_vec4(ptr + wrote, 109, 97, 105, 110)
  return wrote
}

export function has_coredump(): boolean {
    return load<u32>(0) === 0x6d736100;
}

// Start a new frame
export function start_frame(codeoffset: u32, funcidx: u32, local_count: u32): void {
  if (load<u32>(0) === 0x6d736100) {
    // Check for the presence of the wasm\0 header, meaning a coredump
    // is currently being emitted.
    unreachable()
  }

  let ptr = frames_ptr;

  // Create frame struct
  ptr += write_u8(ptr, 0) // frame version 0
  ptr += wasm.write_leb128_u32(ptr, funcidx)
  ptr += wasm.write_leb128_u32(ptr, codeoffset)
  ptr += wasm.write_leb128_u32(ptr, local_count)
  ptr += wasm.write_leb128_u32(ptr, 0) // stack count

  frames_ptr = ptr
  frame_count = frame_count + 1;
}

export function add_missing_local(): void {
  let ptr = frames_ptr;

  // Type
  store<u8>(ptr, 0x01)
  ptr += 1;

  frames_ptr = ptr
}

export function add_i32_local(v: i32): void {
  let ptr = frames_ptr;

  // Type
  store<u8>(ptr, 0x7F)
  ptr += 1;

  // Value
  store<i32>(ptr, v)
  ptr += sizeof<i32>();

  frames_ptr = ptr
}

export function add_f32_local(v: f32): void {
  let ptr = frames_ptr;

  // Type
  store<u8>(ptr, 0x7D)
  ptr += 1;

  // Value
  store<f32>(ptr, v)
  ptr += sizeof<f32>();

  frames_ptr = ptr
}

export function add_f64_local(v: f64): void {
  let ptr = frames_ptr;

  // Type
  store<u8>(ptr, 0x7C)
  ptr += 1;

  // Value
  store<f64>(ptr, v)
  ptr += sizeof<f64>();

  frames_ptr = ptr
}

export function add_i64_local(v: i64): void {
  let ptr = frames_ptr;

  // Type
  store<u8>(ptr, 0x7E)
  ptr += 1;

  // Value
  store<i64>(ptr, v)
  ptr += sizeof<i64>();

  frames_ptr = ptr
}

export function write_coredump(): void {
  // if (load<u32>(0) === 0x6d736100) {
  //   // Check for the presence of the wasm\0 header, meaning a coredump
  //   // would already have been written.
  //   unreachable()
  // }

  let ptr: u32 = 0;

  // End of all the frames, aka their cumulative size.
  let frames_size = frames_ptr

  // copy coredump struct in the corestack section
  const corestack_section_size = frames_size
    + 1 // section name size
    + 9 // section name
    + 1 // thread info type
    + 1 // thread name size
    + 4 // thread name
    + wasm.leb128_u32_byte_size(frame_count) // frame count

  // The `corestack` section contains the coredump stack frames. We wrote the
  // frames in the memory and we construct the Wasm section around them.
  const start_corestack_section =
    4 // wasm header
    + 4 // wasm version
    + 1 // section id
    + wasm.leb128_u32_byte_size(corestack_section_size) // section size
    + 1 // section name size
    + 9 // section name
    + 1 // thread info type
    + 1 // thread name size
    + 4 // thread name
    + wasm.leb128_u32_byte_size(frame_count) // frame count
  memory.copy(start_corestack_section, 0, frames_size);

  // Wasm header
  ptr += wasm.write_magic(ptr);
  ptr += wasm.write_version(ptr);

  {
    ptr += wasm.write_section_header(ptr, 0, corestack_section_size);
    // Section name. Avoids statically allocated strings by writing char manually
    ptr += wasm.write_vec9(ptr, 99, 111, 114, 101, 115, 116, 97, 99, 107)
    ptr += write_thread_info(ptr)

    // Number of frames
    ptr += wasm.write_leb128_u32(ptr, frame_count)

    // the content is followed, it was copied earlier.
    ptr += frames_size
  }

  // core section
  {
    const section_size =
      1 // section name size
      + 4 // section name
      + 1 // process-info type
      + 1 // executeable name size

    ptr += wasm.write_section_header(ptr, 0, section_size);
    // Section name. Avoids statically allocated strings by writing char manually
    ptr += wasm.write_vec4(ptr, 99, 111, 114, 101)
    ptr += write_process_info(ptr)
  }

  // memory section
  {
    const max = memory.size();
    const section_size =
      1 // memory count
      + 1 // memory type
      + 1 // memory min
      + wasm.leb128_u32_byte_size(max) // memory max 

    ptr += wasm.write_section_header(ptr, 5, section_size);
    ptr += wasm.write_leb128_u32(ptr, 1) // memory count
    ptr += wasm.write_memory_with_max(ptr, 0, max)
  }

  // data section
  {
    const mem_size = memory.size() * 64 * 1024;
    let section_size = (mem_size - ptr)
    // Subtract from the section size how many bytes the size itself takes
    section_size = section_size - wasm.leb128_u32_byte_size(section_size) - 1

    ptr += wasm.write_section_header(ptr, 11, section_size);
    ptr += wasm.write_leb128_u32(ptr, 1); // one data segment

    // data segment
    {
      ptr += wasm.write_leb128_u32(ptr, 0); // type active

      const data_start = ptr
        + 1 // i32.const instr
        + wasm.leb128_u32_byte_size(ptr) // i32.const value
        + 1 // end
        + wasm.leb128_u32_byte_size(mem_size - ptr) // vec byte size (approx)
      ptr += wasm.write_i32_const(ptr, data_start)
      ptr += wasm.write_end(ptr)

      const data_len = mem_size - ptr - wasm.leb128_u32_byte_size(mem_size - ptr);
      ptr += wasm.write_leb128_u32(ptr, data_len); // content size
      // rest of the memory is following...
    }
  }
}
