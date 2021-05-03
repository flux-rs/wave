#include <unistd.h>
#include <sys/syscall.h>
#include "wrappers_utils.hpp"
#include "smack.h"
// #include "assert.h"
//All arguments are the Wasm arguments
extern "C" int safe_open(const sandboxptr pathname, int flags){

    hostptr host_pathname = path_from_sandbox(pathname);
    if (host_pathname == NULL)
        return -1;
   
    assert( (host_pathname >= (char*)membase) && (host_pathname + PATH_MAX <= (char*)(membase + memlen)) );
    return syscall(SYS_open, 
        host_pathname, 
        flags, 
        NULL);
}

extern "C" int safe_close(int fd){
    return syscall(SYS_close, 
        fd, 
        NULL);
}

extern "C" ssize_t safe_read(int fd, sandboxptr buf, size_t count){
    hostptr host_buf = sized_buf_from_sandbox(buf, count);
    if (host_buf == NULL)
        return -1;
    
    assert( (host_buf >= (hostptr)membase) && (host_buf + count <= (hostptr)(membase + memlen)) );
    return syscall(SYS_read, 
        fd, 
        host_buf, 
        count, 
        NULL);
}

extern "C" ssize_t safe_write(int fd, const sandboxptr buf, size_t count){
    hostptr host_buf = sized_buf_from_sandbox(buf, count);
    if (host_buf == NULL)
        return -1;

    assert( (host_buf >= (hostptr)membase) && (host_buf + count <= (hostptr)(membase + memlen)) );
    return syscall(SYS_write, 
        fd, 
        host_buf, 
        count, 
        NULL);
}

/*
int main(){

}
*/
