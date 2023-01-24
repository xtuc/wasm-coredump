import * as wasm from 'asc-wasm/assembly'

@inline
function write_u8(ptr: u32, v: u8): u32 {
  store<u8>(ptr, v)
  return 1;
}

export function write_coredump(): void {
  let ptr: u32 = 0;

  // copy coredump struct in the core0 section
  const end_frames_ptr = load<u32>(4);
  const core0_section_size = end_frames_ptr

  // The `corestack` section contains the coredump stack frames. The `set_frame`
  // functions wrote the frames in the memory and we construct the Wasm
  // section around them.
  const start_core0_section =
    4 // wasm header
    + 4 // wasm version
    + 1 // section id
    + wasm.leb128_u32_byte_size(core0_section_size) // section size
    + 1 // section name size
    + 9; // section name
    + 1 // thread name size
    + 4 // thread name
  memory.copy(start_core0_section, 0, end_frames_ptr);

  // Wasm header
  ptr += wasm.write_magic(ptr);
  ptr += wasm.write_version(ptr);

  {
    const section_name_size = 
        1 // vec lengh
      + 9; // chars
    ptr += wasm.write_section_header(ptr, 0, core0_section_size + section_name_size);

    // Section name. Avoids statically allocated strings by writing char manually
    ptr += wasm.write_vec9(ptr, 99, 111, 114, 101, 115, 116, 97, 99, 107)
    ptr += write_u8(ptr, 0x0)
    // Thead name: main.
    ptr += wasm.write_vec4(ptr, 109, 97, 105, 110)

    // the content is followed, it was copied earlier.
  }

  ptr += core0_section_size

  // mem
  {
    const mem_size = memory.size() * 64 * 1024;
    let section_size = (mem_size - ptr)
    // Subtract from the section size how many bytes the size itself takes
    section_size = section_size - wasm.leb128_u32_byte_size(section_size) - 1

    ptr += wasm.write_section_header(ptr, 11, section_size);
    ptr += wasm.write_leb128_u32(ptr, 1); // only data segment

    // data segment
    {
      ptr += wasm.write_leb128_u32(ptr, 1); // type passive
      ptr += wasm.write_leb128_u32(ptr, mem_size - ptr - wasm.leb128_u32_byte_size(mem_size - ptr)); // content size

      // rest of the memory is following...
    }
  }
}
