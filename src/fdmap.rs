#[cfg(feature = "verify")]
use crate::external_specs::vec::*;
use crate::types::*;
use prusti_contracts::*;
use std::io::{stderr, stdin, stdout};
use std::os::unix::io::AsRawFd;
use RuntimeError::*;

/*
Data structure to map sandbox file descriptors to host file descriptors.
We will prove things about it's API as necessary.
*/

impl FdMap {
    // #[trusted]
    // #[ensures (result.m.len() == MAX_SBOX_FDS)]
    #[ensures (result.reserve.len() == 0)]
    #[ensures (result.counter == 0)]
    pub fn new() -> Self {
        FdMap {
            m: vec![Err(Ebadf); MAX_SBOX_FDS as usize],
            reserve: Vec::new(),
            counter: 0,
        }
    }

    #[requires (self.counter == 0)] //should only be called on empty fdmap
    pub fn init_std_fds(&mut self) {
        let stdin_fd = stdin().as_raw_fd() as usize;
        let stdout_fd = stdout().as_raw_fd() as usize;
        let stderr_fd = stderr().as_raw_fd() as usize;
        self.create(stdin_fd.into());
        self.create(stdout_fd.into());
        self.create(stderr_fd.into());
    }

    // Trusted because I can't get the verifier to understand that
    // this can't ever err and it is pretty clear it is correct.
    // Can be fixed with https://viperproject.github.io/prusti-dev/user-guide/verify/pledge.html
    #[trusted]
    #[pure]
    #[requires (index < MAX_SBOX_FDS )]
    pub fn lookup(&self, index: SboxFd) -> RuntimeResult<HostFd> {
        self.m[index as usize]
    }

    // #[trusted]
    fn pop_fd(&mut self) -> RuntimeResult<SboxFd> {
        match self.reserve.pop() {
            Some(fd) => Ok(fd),
            None => {
                if self.counter < MAX_SBOX_FDS {
                    self.counter += 1;
                    return Ok(self.counter - 1);
                }
                Err(Emfile)
            }
        }
    }

    // #[trusted]
    // #[requires(k < MAX_HOST_FDS)]
    // #[ensures (self.lookup(k) == result)]
    // #[ensures (forall(|i: usize| (i < MAX_SBOX_FDS && i != k) ==>
    //                 self.lookup(i) == old(self.lookup(i))))]
    pub fn create(&mut self, k: HostFd) -> RuntimeResult<SboxFd> {
        let s_fd = self.pop_fd()?;
        self.m[s_fd as usize] = Ok(k);
        Ok(s_fd)
    }

    // #[trusted]
    #[requires(k < MAX_SBOX_FDS)]
    // #[ensures (self.lookup(k).is_err())]
    // #[ensures (forall(|i: usize| (i < MAX_SBOX_FDS && i != k) ==>
    //                 self.lookup(i) == old(self).lookup(i)))]
    pub fn delete(&mut self, k: SboxFd) {
        if let Ok(oldfd) = self.m[k as usize] {
            self.reserve.push(k);
        }
        self.m[k as usize] = Err(Ebadf);
    }
}
