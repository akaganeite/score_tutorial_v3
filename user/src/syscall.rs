use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {//asm! 宏可以将汇编代码嵌入到局部的函数上下文中
        asm!(//x10-x17对应a0-a7，a0-a6保存系统调用参数
            "ecall",
            inlateout("x10") args[0] => ret,//a0保存系统调用返回值
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id  //syscall_id
        );
    }
    ret
}

/// 功能：将内存中缓冲区中的数据写入文件。
/// 参数：`fd` 表示待写入文件的文件描述符；
///      `buf` 表示内存中缓冲区的起始地址；
///      `len` 表示内存中缓冲区的长度。
/// 返回值：返回成功写入的长度。
/// syscall ID：64
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {//&[]是切片，存有地址和长度，通过asptr和len取出
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 功能：退出应用程序并将返回值告知批处理系统。
/// 参数：`exit_code` 表示应用程序的返回值。
/// 返回值：该系统调用不应该返回。
/// syscall ID：93
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
