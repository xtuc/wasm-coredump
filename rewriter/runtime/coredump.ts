import * as wasm from 'asc-wasm/assembly'

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

export function write_coredump(): void {
  let ptr: u32 = 0;

  // copy coredump struct in the corestack section
  const frames_size = load<u32>(4);
  const corestack_section_size = frames_size
    + 1 // section name size
    + 9 // section name
    + 1 // thread info type
    + 1 // thread name size
    + 4 // thread name

  // The `corestack` section contains the coredump stack frames. The `set_frame`
  // functions wrote the frames in the memory and we construct the Wasm
  // section around them.
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
  memory.copy(start_corestack_section, 0, frames_size);

  // Wasm header
  ptr += wasm.write_magic(ptr);
  ptr += wasm.write_version(ptr);

  {
    ptr += wasm.write_section_header(ptr, 0, corestack_section_size);
    // Section name. Avoids statically allocated strings by writing char manually
    ptr += wasm.write_vec9(ptr, 99, 111, 114, 101, 115, 116, 97, 99, 107)
    ptr += write_thread_info(ptr)

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
