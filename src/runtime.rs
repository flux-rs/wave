#[cfg(feature = "verify")]
use crate::verifier::external_specs::option::*;
//use crate::spec::trace_safe;
//use crate::trace::Trace;
use crate::types::*;
#[cfg(feature = "verify")]
use crate::verifier::*;
use crate::{effect, four_effects, no_effect, one_effect, three_effects, two_effects};
use extra_args::{external_call, external_method, with_ghost_var};
use prusti_contracts::*;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::{Component, Path, PathBuf};
use std::ptr::{copy, copy_nonoverlapping};

use RuntimeError::*;

//#[ensures(safe(&result))]
#[with_ghost_var(trace: &mut Trace)]
#[external_call(new)]
#[external_method(init_std_fds)]
pub fn fresh_ctx(homedir: String) -> VmCtx {
    let memlen = LINEAR_MEM_SIZE;
    let mem = vec![0; memlen];
    let mut fdmap = FdMap::new();
    fdmap.init_std_fds();
    let arg_buffer = Vec::new();
    let argc = 0;
    let env_buffer = Vec::new();
    let envc = 0;

    VmCtx {
        mem,
        memlen,
        fdmap,
        homedir,
        errno: Success,
        arg_buffer,
        argc,
        env_buffer,
        envc,
    }
}

impl VmCtx {
    /// Check whether sandbox pointer is actually inside the sandbox
    #[with_ghost_var(trace: &mut Trace)]
    #[pure]
    #[ensures((result == true) ==> (ptr as usize) < self.memlen)]
    pub fn in_lin_mem(&self, ptr: SboxPtr) -> bool {
        (ptr as usize) < self.memlen
    }

    // TODO: does this have to be trusted?
    #[with_ghost_var(trace: &mut Trace)]
    #[requires(self.fits_in_lin_mem(ptr, len, trace))]
    #[ensures(result.len() == (len as usize))]
    #[after_expiry(
        self.memlen == old(self.memlen))]
    #[trusted]
    pub fn slice_mem(&self, ptr: SboxPtr, len: u32) -> &[u8] {
        let start = ptr as usize;
        let end = ptr as usize + len as usize;
        &self.mem[start..end]
    }

    // TODO: does this have to be trusted?
    #[with_ghost_var(trace: &mut Trace)]
    #[requires(self.fits_in_lin_mem(ptr, len, trace))]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    #[ensures(result.len() == (len as usize))]
    #[after_expiry(
        self.memlen == old(self.memlen))]
    #[trusted]
    pub fn slice_mem_mut(&mut self, ptr: SboxPtr, len: u32) -> &mut [u8] {
        let start = ptr as usize;
        let end = ptr as usize + len as usize;
        &mut self.mem[start..end]
    }

    /// Check whether buffer is entirely within sandbox
    #[pure]
    #[with_ghost_var(trace: &mut Trace)]
    #[ensures(result == true ==> (buf as usize) < self.memlen && ((buf + cnt) as usize) < self.memlen && (cnt as usize) < self.memlen)]
    pub fn fits_in_lin_mem(&self, buf: SboxPtr, cnt: u32) -> bool {
        let total_size = (buf as usize) + (cnt as usize);
        if total_size > self.memlen || total_size > LINEAR_MEM_SIZE {
            return false;
        }
        self.in_lin_mem(buf) && self.in_lin_mem(cnt) && self.in_lin_mem(buf + cnt)
    }

    /// Copy buffer from sandbox to host
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(new)]
    #[external_method(reserve_exact)]
    #[requires(self.fits_in_lin_mem(src, n, trace))]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    #[ensures(result.len() == (n as usize) )]
    pub fn copy_buf_from_sandbox(&self, src: SboxPtr, n: u32) -> Vec<u8> {
        let mut host_buffer: Vec<u8> = Vec::new();
        host_buffer.reserve_exact(n as usize);
        self.memcpy_from_sandbox(&mut host_buffer, src, n);
        host_buffer
    }

    /// Copy buffer from from host to sandbox
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(Some)]
    #[requires(src.len() == (n as usize) )]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    #[ensures(self.memlen == old(self.memlen))]
    pub fn copy_buf_to_sandbox(&mut self, dst: SboxPtr, src: &Vec<u8>, n: u32) -> Option<()> {
        if !self.fits_in_lin_mem(dst, n) {
            return None;
        }
        self.memcpy_to_sandbox(dst, src, n);
        Some(())
    }

    /// Copy arg buffer from from host to sandbox
    /// TODO: make this not trusted
    /// (its only trusted because clone breaks viper for some reason)
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(Some)]
    #[trusted]
    #[requires(self.arg_buffer.len() == (n as usize) )]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    pub fn copy_arg_buffer_to_sandbox(&mut self, dst: SboxPtr, n: u32) -> Option<()> {
        if !self.fits_in_lin_mem(dst, n) {
            return None;
        }
        self.memcpy_to_sandbox(dst, &self.arg_buffer.clone(), n);
        Some(())
    }

    /// Copy arg buffer from from host to sandbox
    /// TODO: make this not trusted
    /// (its only trusted because clone breaks viper for some reason)
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(Some)]
    #[trusted]
    #[requires(self.env_buffer.len() == (n as usize) )]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    pub fn copy_environ_buffer_to_sandbox(&mut self, dst: SboxPtr, n: u32) -> Option<()> {
        if !self.fits_in_lin_mem(dst, n) {
            return None;
        }
        self.memcpy_to_sandbox(dst, &self.env_buffer.clone(), n);
        Some(())
    }

    /// Function for memcpy from sandbox to host
    /// Overwrites contents of vec
    /// One of 2 unsafe functions (besides syscalls), so needs to be obviously correct
    //TODO: verify that regions do not overlap so that we can use copy_non_overlapping
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(copy)]
    #[external_method(set_len)]
    #[trusted]
    #[requires(dst.capacity() >= (n as usize) )]
    #[requires(self.fits_in_lin_mem(src, n, trace))]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    #[ensures(dst.len() == (n as usize) )]
    #[ensures(self.memlen == old(self.memlen))]
    pub fn memcpy_from_sandbox(&self, dst: &mut Vec<u8>, src: SboxPtr, n: u32) {
        unsafe {
            copy(
                self.mem.as_ptr().offset(src as isize),
                dst.as_mut_ptr(),
                n as usize,
            );
            dst.set_len(n as usize);
        };
    }

    /// Function for memcpy from sandbox to host
    /// One of 2 unsafe functions (besides syscalls), so needs to be obviously correct
    //TODO: verify that regions do not overlap so that we can use copy_non_overlapping
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(copy)]
    #[trusted]
    #[requires(src.len() == (n as usize) )]
    #[requires(self.fits_in_lin_mem(dst, n, trace))]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    #[ensures(old(self.memlen) == self.memlen)]
    // #[requires(dst < (self.memlen as u32) )]
    // #[requires(dst + n < (self.memlen as u32) )]
    pub fn memcpy_to_sandbox(&mut self, dst: SboxPtr, src: &Vec<u8>, n: u32) {
        unsafe {
            copy(
                src.as_ptr(),
                self.mem.as_mut_ptr().offset(dst as isize),
                n as usize,
            )
        };
    }

    /// Check whether a path is in the home directory.
    /// If it is, return it as an absolute path, if it isn't, return error
    // TODO: verify and make untrusted
    // #[with_ghost_var(trace: &mut Trace)]
    #[trusted]
    // #[requires(trace_safe(self, trace))]
    // #[ensures(trace_safe(self, trace))]
    pub fn resolve_path(&self, in_path: Vec<u8>) -> RuntimeResult<SandboxedPath> {
        let path = PathBuf::from(OsString::from_vec(in_path));
        let safe_path = normalize_path(&path);
        let path_str = safe_path.into_os_string();
        if let Ok(s) = path_str.into_string() {
            if self.homedir.starts_with(&s) {
                return Ok(SandboxedPath::from(s.into_bytes()));
            }
        }
        Err(Eacces)
    }

    /// Check whether a path is relative.
    /// If it is, return it as a relative path. If it isn't, return an error
    // TODO: should a relative path really be an (fd, path) tuple? i.e. whenever we use them,
    //       they always have an associated Fd that they are relative to.
    //       See wasi_path_create_directory for an example
    /*pub fn ensure_relative_path(&self, in_path: Vec<u8>) -> RuntimeResult<RelativePath> {
    #[trusted]
    pub fn ensure_relative_path(&self, in_path: Vec<u8>) -> RuntimeResult<RelativePath> {
        let path = PathBuf::from(OsString::from_vec(in_path));
        if !path.is_relative() {
            return Err(Eacces);
        }
        let path_str = path.into_os_string();
        if let Ok(s) = path_str.into_string() {
            return Ok(RelativePath::from(s.into_bytes()));
        }
        Err(Eacces)
    }*/

    /// read u16 from wasm linear memory
    // Not thrilled about this implementation, but it works
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(from_le_bytes)]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    pub fn read_u16(&self, start: usize) -> u16 {
        let bytes: [u8; 2] = [self.mem[start], self.mem[start + 1]];
        u16::from_le_bytes(bytes)
    }

    /// read u32 from wasm linear memory
    // Not thrilled about this implementation, but it works
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(from_le_bytes)]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    pub fn read_u32(&self, start: usize) -> u32 {
        let bytes: [u8; 4] = [
            self.mem[start],
            self.mem[start + 1],
            self.mem[start + 2],
            self.mem[start + 3],
        ];
        u32::from_le_bytes(bytes)
    }

    /// read u64 from wasm linear memory
    // Not thrilled about this implementation, but it works
    #[with_ghost_var(trace: &mut Trace)]
    #[external_call(from_le_bytes)]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    pub fn read_u64(&self, start: usize) -> u64 {
        let bytes: [u8; 8] = [
            self.mem[start],
            self.mem[start + 1],
            self.mem[start + 2],
            self.mem[start + 3],
            self.mem[start + 4],
            self.mem[start + 5],
            self.mem[start + 6],
            self.mem[start + 7],
        ];
        u64::from_le_bytes(bytes)
    }

    /// write u16 to wasm linear memory
    // Not thrilled about this implementation, but it works
    #[with_ghost_var(trace: &mut Trace)]
    #[external_method(to_le_bytes)]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    pub fn write_u16(&mut self, start: usize, v: u16) {
        let bytes: [u8; 2] = v.to_le_bytes();
        self.mem[start] = bytes[0];
        self.mem[start + 1] = bytes[1];
    }

    /// write u32 to wasm linear memory
    // Not thrilled about this implementation, but it works
    #[with_ghost_var(trace: &mut Trace)]
    #[external_method(to_le_bytes)]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    pub fn write_u32(&mut self, start: usize, v: u32) {
        let bytes: [u8; 4] = v.to_le_bytes();
        self.mem[start] = bytes[0];
        self.mem[start + 1] = bytes[1];
        self.mem[start + 2] = bytes[2];
        self.mem[start + 3] = bytes[3];
    }

    /// write u64 to wasm linear memory
    // Not thrilled about this implementation, but it works
    #[with_ghost_var(trace: &mut Trace)]
    #[external_method(to_le_bytes)]
    // #[requires(self.memlen > 8)]
    #[requires(trace_safe(self, trace))]
    #[ensures(trace_safe(self, trace))]
    // #[ensures(one_effect!(old(trace), trace, Effect::WriteN(8)))]
    pub fn write_u64(&mut self, start: usize, v: u64) {
        let bytes: [u8; 8] = v.to_le_bytes();
        self.mem[start] = bytes[0];
        self.mem[start + 1] = bytes[1];
        self.mem[start + 2] = bytes[2];
        self.mem[start + 3] = bytes[3];
        self.mem[start + 4] = bytes[4];
        self.mem[start + 5] = bytes[5];
        self.mem[start + 6] = bytes[6];
        self.mem[start + 7] = bytes[7];
        // effect!(trace, Effect::WriteN(8));
    }
}

/// Convert relative path to absolute path
/// Used to check that that paths are sandboxed
// TODO: verify this
// Prusti does not like this function at all
#[trusted]
pub fn normalize_path(path: &PathBuf) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
