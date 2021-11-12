# Rust Version Script Testing
This project is for debugging how version scripts can be used in Rust. For testing purposes, it attempts to export the
function `foo` (found in `src/lib.rs`) under the version `testing1.0` (found in `./version_script`).

## Testing
If the build successfully used the version script, you should be able to use `objdump` or `readelf` to verify that `foo`
was globally exported under the version `testing1.0`.

```bash
$ cargo build
   Compiling version_script_test v0.1.0 (/home/tests/projects/version_script_test)
    Finished dev [unoptimized + debuginfo] target(s) in 15.09s
$ objdump -TC target/debug/libtesting.so
testing.so

target/debug/libtesting.so:     file format elf64-x86-64

DYNAMIC SYMBOL TABLE:
0000000000000000  w   D  *UND*  0000000000000000  Base        __gmon_start__
0000000000000000  w   D  *UND*  0000000000000000  Base        _ITM_deregisterTMCloneTable
0000000000000000  w   D  *UND*  0000000000000000  Base        _ITM_registerTMCloneTable
0000000000000000  w   DF *UND*  0000000000000000  Base        __cxa_finalize
0000000000000000      DF *UND*  0000000000000000  GLIBC_2.2.5 pthread_mutex_lock
0000000000000000      DF *UND*  0000000000000000  GLIBC_2.2.5 pthread_mutex_unlock
0000000000000000      DF *UND*  0000000000000000  GCC_3.0     _Unwind_GetDataRelBase
0000000000000000      DF *UND*  0000000000000000  GCC_4.2.0   _Unwind_GetIPInfo
0000000000000000      DF *UND*  0000000000000000  GCC_3.0     _Unwind_GetLanguageSpecificData
0000000000000000      DF *UND*  0000000000000000  GCC_3.0     _Unwind_GetRegionStart
0000000000000000      DF *UND*  0000000000000000  GCC_3.0     _Unwind_GetTextRelBase
0000000000000000      DF *UND*  0000000000000000  GCC_3.0     _Unwind_SetGR
0000000000000000      DF *UND*  0000000000000000  GCC_3.0     _Unwind_SetIP
0000000000003c40 g    DF .text  00000000000002e1  Base        rust_eh_personality
0000000000003a10 g    D  .text  0000000000000000  testing1.0  foo
```