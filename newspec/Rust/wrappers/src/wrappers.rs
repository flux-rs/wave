use smack::*;
use crate::runtime::*;
use crate::os::*;
use std::convert::TryInto;

// use crate::runtime::delete_seal;
// predicate SFISafe(ctx) =
// not exists. a. a < ctx.membase | a >= ctx.membase + ctx.memlength. access(a)

// predicate FdSafe(ctx) =
// not exists. fd. inRevFdMap(ctx, fd) & os_read_fd(fd)

// validctx(ctx):
// ctx.membase < ctx.membase + ctx.membaseLen
// forall fd. inRevFdMap(ctx fd) => inFdMap(ctx, translateFd(ctx, fd))
// forall vfd. inFdMap(ctx vfd) => inRevFdMap(ctx, translateFd(ctx, vfd))

// WASIRead(ctx): ... write at most v_cnt bytes etc.

// validCtx(ctx), SFISafe(ctx), FdSafe(ctx) = ...


//pre: {..., }
//post: {..., inFDMap(ctx, fd), inRevFDMap(ctx, translate_fd(fd) )}
// pub fn wasi_open(ctx: &VmCtx, const sandboxptr pathname, int flags) -> i32 {
//     let host_buffer = copy_buf_from_sandbox(ctx, pathname, PATH_MAX);
//     if (host_buffer == NULL){
//         return -1;
//     }
//     let host_pathname = malloc(PATH_MAX);
//     resolve_path(ctx, host_buffer, host_pathname);
//     let fd = os_open(host_pathname, flags);
    
//     let sbox_fd = create_seal(ctx, fd, ctx->counter++);
//     // free(host_buffer);
//     // free(host_pathname);
//     return sbox_fd;
// }

//pre: {...}
//post: {..., !inFDMap(ctx, fd), !inRevFDMap(ctx, translate_fd(fd) )}
pub fn wasi_close(ctx: &mut VmCtx, v_fd: SboxFd) -> i32 {
    if (v_fd < 0) || (v_fd >= MAX_SANDBOX_FDS) || !in_fd_map(ctx, v_fd){
        return -1;
    }
    let fd = translate_fd(ctx, v_fd);
    delete_seal(ctx, v_fd);
    return os_close(fd);
    // return 1;
}

// // pre: { validCtx(ctx)}
// // post: { validCtx(ctx), SFISafe(ctx), FdSafe(ctx), WASIRead(ctx) }
// pub fn wasi_read(ctx &VmCtx, int v_fd, sandboxptr v_buf, v_cnt: usize) -> size {
//   let buf = swizzle(ctx, v_buf);

//   if (!inMemRegion(ctx, buf) || (v_cnt >= ctx->memlen) || !fitsInMemRegion(ctx, buf, v_cnt)){
//       return -1;
//   }
//   if (v_fd < 0 || v_fd >= MAX_SANDBOX_FDS || !in_fd_map(ctx, v_fd)){
//         return -1;
//   }
//   let fd = translate_fd(ctx, v_fd);
//   return os_read(fd, buf, v_cnt);
// }


pub fn wasi_write(ctx: &VmCtx, v_fd: SboxFd, v_buf: SboxPtr, v_cnt: usize) -> isize {
  //void *buf = swizzle(ctx, v_buf);
  let buf = swizzle(ctx, v_buf);

  if !in_mem_region(ctx, buf) || ((v_cnt as usize) >= ctx.memlen) || !fits_in_mem_region(ctx, buf, v_cnt){
      return -1;
  }
  if v_fd < 0 || v_fd >= MAX_SANDBOX_FDS || !in_fd_map(ctx, v_fd){
        return -1;
  }
  let fd = translate_fd(ctx, v_fd);
  return os_write(fd, buf as *mut u8, v_cnt);
}

