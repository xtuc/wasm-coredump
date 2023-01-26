let out = "";

out += `
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
`;

function generate(nargs) {
  const args = Array(nargs)
    .fill(0)
    .map((_, i) => `arg${i}: u32`)
    .join(", ");

  const setLocals = Array(nargs)
    .fill(0)
    .map((_, i) => `store<u32>(base, arg${i}, ${(2 * 4) + i * 4}, 1)`)
    .join("\n");

  const template = `
export function set_frame${nargs}(code_offset: u32, ${args}): void {
  let base = get_next_frame_ptr();

  // Local count is the number of arguments in set_frame*. The caller passes
  // function locals as arguments.
  const local_count = ${nargs}

  // Create frame struct
  store<u32>(base, code_offset, 0, 1)
  store<u32>(base, local_count, 4, 1)
  // Set locals
${setLocals}

  // update frame counter
  const frame_count = load<u32>(0, 0, 1)
  store<u32>(0, frame_count + 1, 0, 1)
  // Update next frame addr
  store<u32>(4, base + ${(2 * 4) + (nargs * 4)}, 0, 1)
}
`;

  out += template;
}

for (let i = 0; i <= 30; i++) {
  generate(i);
}

console.log(out);
