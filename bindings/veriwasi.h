#ifndef veriwasi_bindings_h
#define veriwasi_bindings_h

/* Generated with cbindgen:0.20.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define MAX_SBOX_FDS 8

#define MAX_HOST_FDS 1024

#define PATH_MAX 1024

#define PAGE_SIZE 4096

#define LINEAR_MEM_SIZE 4294965096

typedef struct VmCtx VmCtx;

typedef uint32_t SboxFd;

#define HOMEDIR_FD 3

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct VmCtx *veriwasi_init(uint8_t *memptr,
                            intptr_t memsize,
                            const char *homedir,
                            uint8_t *args,
                            uintptr_t argc,
                            uint8_t *env,
                            uintptr_t envc,
                            char *log_path);

uint32_t Z_wasi_snapshot_preview1Z_args_getZ_iii(void *ctx,
                                                 uint32_t argv,
                                                 uint32_t argv_buf);

uint32_t Z_wasi_snapshot_preview1Z_args_sizes_getZ_iii(void *ctx,
                                                       uint32_t pargc,
                                                       uint32_t pargv_buf_size);

void Z_wasi_snapshot_preview1Z_proc_exitZ_vi(void *ctx,
                                             uint32_t x);

uint32_t Z_wasi_snapshot_preview1Z_environ_sizes_getZ_iii(void *ctx,
                                                          uint32_t pcount,
                                                          uint32_t pbuf_size);

uint32_t Z_wasi_snapshot_preview1Z_environ_getZ_iii(void *ctx,
                                                    uint32_t __environ,
                                                    uint32_t environ_buf);

uint32_t Z_wasi_snapshot_preview1Z_fd_prestat_getZ_iii(void *ctx,
                                                       uint32_t fd,
                                                       uint32_t prestat);

uint32_t Z_wasi_snapshot_preview1Z_fd_writeZ_iiiii(void *ctx,
                                                   uint32_t fd,
                                                   uint32_t iov,
                                                   uint32_t iovcnt,
                                                   uint32_t pnum);

uint32_t Z_wasi_snapshot_preview1Z_fd_readZ_iiiii(void *ctx,
                                                  uint32_t fd,
                                                  uint32_t iov,
                                                  uint32_t iovcnt,
                                                  uint32_t pnum);

uint32_t Z_wasi_snapshot_preview1Z_fd_closeZ_ii(void *ctx,
                                                uint32_t fd);

uint32_t Z_wasi_snapshot_preview1Z_fd_seekZ_iijii(void *ctx,
                                                  uint32_t fd,
                                                  uint64_t offset,
                                                  uint32_t whence,
                                                  uint32_t new_offset);

uint32_t Z_wasi_snapshot_preview1Z_clock_time_getZ_iiji(void *ctx,
                                                        uint32_t clock_id,
                                                        uint64_t precision,
                                                        uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_clock_res_getZ_iii(void *ctx,
                                                      uint32_t clock_id,
                                                      uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_fd_adviseZ_iijji(void *ctx,
                                                    uint32_t v_fd,
                                                    uint64_t offset,
                                                    uint64_t len,
                                                    uint32_t advice);

uint32_t Z_wasi_snapshot_preview1Z_fd_allocateZ_iijj(void *ctx,
                                                     uint32_t v_fd,
                                                     uint64_t offset,
                                                     uint64_t len);

uint32_t Z_wasi_snapshot_preview1Z_fd_datasyncZ_ii(void *ctx,
                                                   uint32_t v_fd);

uint32_t Z_wasi_snapshot_preview1Z_fd_fdstat_getZ_iii(void *ctx,
                                                      uint32_t v_fd,
                                                      uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_fd_fdstat_set_flagsZ_iii(void *ctx,
                                                            uint32_t v_fd,
                                                            uint32_t flags);

uint32_t Z_wasi_snapshot_preview1Z_fd_filestat_getZ_iii(void *ctx,
                                                        uint32_t v_fd,
                                                        uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_fd_filestat_set_sizeZ_iij(void *ctx,
                                                             uint32_t v_fd,
                                                             uint64_t size);

uint32_t Z_wasi_snapshot_preview1Z_fd_filestat_set_timesZ_iijji(void *ctx,
                                                                uint32_t v_fd,
                                                                uint64_t atim,
                                                                uint64_t mtim,
                                                                uint32_t fst_flags);

uint32_t Z_wasi_snapshot_preview1Z_fd_preadZ_iiiiji(void *ctx,
                                                    uint32_t fd,
                                                    uint32_t iovs,
                                                    uint32_t iov_len,
                                                    uint64_t offset,
                                                    uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_fd_prestat_dir_nameZ_iiii(void *ctx,
                                                             uint32_t fd,
                                                             uint32_t path,
                                                             uint32_t path_len);

uint32_t Z_wasi_snapshot_preview1Z_fd_pwriteZ_iiiiji(void *ctx,
                                                     uint32_t fd,
                                                     uint32_t iovs,
                                                     uint32_t iov_len,
                                                     uint64_t offset,
                                                     uint32_t retptr);

uint32_t Z_wasi_snapshot_preview1Z_fd_readdirZ_iiiiji(void *ctx,
                                                      uint32_t fd,
                                                      uint32_t buf,
                                                      uint32_t buf_len,
                                                      uint64_t cookie,
                                                      uint32_t retptr);

uint32_t Z_wasi_snapshot_preview1Z_fd_renumberZ_iii(void *ctx,
                                                    uint32_t from,
                                                    uint32_t to);

uint32_t Z_wasi_snapshot_preview1Z_fd_syncZ_ii(void *ctx,
                                               uint32_t fd);

uint32_t Z_wasi_snapshot_preview1Z_fd_tellZ_iii(void *ctx,
                                                uint32_t fd,
                                                uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_path_create_directoryZ_iiii(void *ctx,
                                                               uint32_t fd,
                                                               uint32_t pathname,
                                                               uint32_t path_len);

uint32_t Z_wasi_snapshot_preview1Z_path_filestat_getZ_iiiiii(void *ctx,
                                                             uint32_t fd,
                                                             uint32_t flags,
                                                             uint32_t path,
                                                             uint32_t path_len,
                                                             uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_path_filestat_set_timesZ_iiiiijji(void *ctx,
                                                                     uint32_t fd,
                                                                     uint32_t flags,
                                                                     uint32_t path,
                                                                     uint32_t path_len,
                                                                     uint64_t atim,
                                                                     uint64_t mtim,
                                                                     uint32_t fst_flags);

uint32_t Z_wasi_snapshot_preview1Z_path_linkZ_iiiiiiii(void *ctx,
                                                       uint32_t old_fd,
                                                       uint32_t old_flags,
                                                       uint32_t old_path,
                                                       uint32_t old_path_len,
                                                       uint32_t new_fd,
                                                       uint32_t new_path,
                                                       uint32_t new_path_len);

uint32_t Z_wasi_snapshot_preview1Z_path_openZ_iiiiiijjii(void *ctx,
                                                         uint32_t fd,
                                                         uint32_t dirflags,
                                                         uint32_t path,
                                                         uint32_t path_len,
                                                         uint32_t oflags,
                                                         uint64_t fs_rights_base,
                                                         uint64_t _fs_rights_inheriting,
                                                         uint32_t fdflags,
                                                         uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_path_readlinkZ_iiiiiii(void *ctx,
                                                          uint32_t fd,
                                                          uint32_t path,
                                                          uint32_t path_len,
                                                          uint32_t buf,
                                                          uint32_t buf_len,
                                                          uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_path_remove_directoryZ_iiii(void *ctx,
                                                               uint32_t fd,
                                                               uint32_t path,
                                                               uint32_t path_len);

uint32_t Z_wasi_snapshot_preview1Z_path_renameZ_iiiiiii(void *ctx,
                                                        uint32_t old_fd,
                                                        uint32_t old_path,
                                                        uint32_t old_path_len,
                                                        uint32_t new_fd,
                                                        uint32_t new_path,
                                                        uint32_t new_path_len);

uint32_t Z_wasi_snapshot_preview1Z_path_symlinkZ_iiiiii(void *ctx,
                                                        uint32_t old_path,
                                                        uint32_t old_path_len,
                                                        uint32_t fd,
                                                        uint32_t path,
                                                        uint32_t path_len);

uint32_t Z_wasi_snapshot_preview1Z_path_unlink_fileZ_iiii(void *ctx,
                                                          uint32_t fd,
                                                          uint32_t path,
                                                          uint32_t path_len);

uint32_t Z_wasi_snapshot_preview1Z_poll_oneoffZ_iiiii(void *ctx,
                                                      uint32_t in_ptr,
                                                      uint32_t out_ptr,
                                                      uint32_t nsubscriptions,
                                                      uint32_t retptr);

uint32_t Z_wasi_snapshot_preview1Z_proc_raiseZ_ii(void *ctx,
                                                  uint32_t signal);

uint32_t Z_wasi_snapshot_preview1Z_random_getZ_iii(void *ctx,
                                                   uint32_t buf,
                                                   uint32_t buf_len);

uint32_t Z_wasi_snapshot_preview1Z_sched_yieldZ_i(void *ctx);

uint32_t Z_wasi_snapshot_preview1Z_sock_recvZ_iiiiiii(void *ctx,
                                                      uint32_t fd,
                                                      uint32_t ri_data,
                                                      uint32_t ri_data_count,
                                                      uint32_t ri_flags,
                                                      uint32_t out0,
                                                      uint32_t out1);

uint32_t Z_wasi_snapshot_preview1Z_sock_sendZ_iiiiii(void *ctx,
                                                     uint32_t fd,
                                                     uint32_t si_data,
                                                     uint32_t si_data_count,
                                                     uint32_t si_flags,
                                                     uint32_t out);

uint32_t Z_wasi_snapshot_preview1Z_sock_shutdownZ_iii(void *ctx,
                                                      uint32_t fd,
                                                      uint32_t how);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* veriwasi_bindings_h */
