// Pointer or cursor to the latest frame
// Assumed to be globalidx 0 by bin/rewriter/src/runtime.rs
export var frames_ptr: u32 = 0;

// Keep track of number of frames
// Assumed to be globalidx 1 by bin/rewriter/src/runtime.rs
export var frame_count: u32 = 0;

// Make sure the globals are before any imports that might add globals too.
// The rewriter assumes frames_ptr and frame_count are placed first.

// TODO: remove the export, it's just for asc to keep them.

import { prestat, fd, prestat_dir, preopentype, errno, fd_prestat_dir_name, fdflags, fd_write, path_open, oflags, rights } from "@assemblyscript/wasi-shim/assembly/bindings/wasi_snapshot_preview1";

/** Return a description of the given preopened file descriptor. */
// @ts-ignore: decorator
@unsafe
export declare function fd_prestat_get(
  /** Input: The file descriptor about which to retrieve information. */
  fd: fd,
  /** Input: The buffer where the description is stored. */
  buf: prestat_dir
): errno;


const COREDUMP_LOCATION = "/coredump"

function get_fd_for_folder(folderName: string): i32 {
  const utf8FolderName = String.UTF8.encode(folderName);

  // Start from fd 3 (stdin, stdout, stderr are 0-2)
  for (let fd: u32 = 3;; fd++) {
    const prestat = new prestat_dir();
    const prestatResult = fd_prestat_get(fd, prestat);

    // If no more pre-opened FDs, break out of the loop
    if (prestatResult != 0) {
      break;
    }

    // Check if the FD is a directory
    if (prestat.type == preopentype.DIR) {
      const nameBuffer = heap.alloc(prestat.name_len as i32 + 1); // +1 for null-termination
      if (nameBuffer == 0) {
        unreachable()
      }

      const result = fd_prestat_dir_name(fd, nameBuffer, prestat.name_len as i32);
      if (result == 0) {
        const dirName = String.UTF8.decodeUnsafe(nameBuffer, prestat.name_len, true);
        heap.free(nameBuffer);

        if (dirName == folderName) {
          return fd;
        }
      }
    }
  }

  return -1;
}

function append_file(fd: fd, data: u8[]): void {
  let data_buf_len = data.length;
  let data_buf_out = changetype<usize>(new ArrayBuffer(data_buf_len));
  // @ts-ignore: cast
  let data_buf_in = changetype<ArrayBufferView>(data).dataStart;
  memory.copy(data_buf_out, data_buf_in, data_buf_len);
  let iov = memory.data(16);
  store<u32>(iov, data_buf_out, 0);
  store<u32>(iov, data_buf_len, sizeof<usize>());
  let written_ptr = memory.data(8);
  fd_write(fd, iov, 1, written_ptr);
}

export function write_coredump(): void {
  wasi_console.error("core dumped");
  const folder = get_fd_for_folder(COREDUMP_LOCATION)
  if (folder === -1) {
    wasi_console.error(`folder ${COREDUMP_LOCATION} not provided.`);
    unreachable()
  }

  const dirflags: fdflags = 0;
  const path_utf8_buf = String.UTF8.encode("coredump");
  const path_utf8_len: usize = path_utf8_buf.byteLength;
  const path_utf8 = changetype<usize>(path_utf8_buf);
  const fd_rights = rights.FD_WRITE;
  const fd_rights_inherited = fd_rights;
  const openflags: u16 = oflags.CREAT;
  const fd_flags: fdflags = 0;
  const fd_buf = memory.data(8);

  const res = path_open(folder, dirflags, path_utf8, path_utf8_len, openflags, fd_rights, fd_rights_inherited, fd_flags, fd_buf);
  if (res !== errno.SUCCESS) {
    wasi_console.error(`failed to open ${COREDUMP_LOCATION}/coredump: ${res}.`);
    unreachable()
  }
  let fd = load<u32>(fd_buf);

  append_file(fd, [1,2,3]);

  unreachable()
}

export function start_frame(codeoffset: u32, funcidx: u32, local_count: u32): void {
  unreachable()
}

export function add_i32_local(v: i32): void {
  unreachable()
}

export function add_f32_local(v: f32): void {
  unreachable()
}

export function add_f64_local(v: f64): void {
  unreachable()
}

export function add_i64_local(v: i64): void {
  unreachable()
}

export function has_coredump(): boolean {
  return load<u32>(0) === 0x6d736100;
}
