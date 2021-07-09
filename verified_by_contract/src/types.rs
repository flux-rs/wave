use prusti_contracts::*;

pub const MAX_SBOX_FDS: usize = 8;
pub const MAX_HOST_FDS: usize = 1024;
pub const PATH_MAX: usize = 1024;

pub const PAGE_SIZE: usize = 4096;
pub const LINEAR_MEM_SIZE: usize = 4294965096; //4GB
                                               // #define SFI_SAFE(ctx) (true) //This is handled by the builtin memory safety checker

// #define FD_SAFE(ctx) (true) // Unimplemented - I think I want to move to rust for better types to implement this
// #define PATH_SAFE(ctx) (true) // Unimplemented - I think I want to move to rust for better types to implement this
// #define RESOURCE_SAFE(ctx) FD_SAFE(ctx) && PATH_SAFE(ctx)

// #define SAFE(ctx) VALID_CTX(ctx) && SFI_SAFE(ctx) && RESOURCE_SAFE(ctx)

//typedef char* hostptr;
pub type HostPtr = usize;
pub type SboxPtr = u32;

pub type HostFd = usize;
pub type SboxFd = usize;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RuntimeError {
    Ebadf,
    Emfile, // process ran out of file descriptors
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct FdMap {
    pub m: Vec<RuntimeResult<HostFd>>,
    pub reserve: Vec<SboxFd>,
    pub counter: SboxFd,
}

pub struct VmCtx {
    pub mem: Vec<u8>,
    pub memlen: usize,
    pub fdmap: FdMap,
}
