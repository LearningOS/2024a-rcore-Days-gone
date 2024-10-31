//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE},
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        TASK_MANAGER,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let timeval = TimeVal {
        sec: get_time_us() / 1_000_000,
        usec: get_time_us() % 1_000_000,
    };
    let va = _ts as usize;
    TASK_MANAGER.cur_task_translate(va).map(|pa| unsafe {
        (pa as *mut TimeVal).write(timeval);
    });
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let (status, syscall_times, time) = TASK_MANAGER.get_current_taskinfo();
    let va = _ti as usize;
    TASK_MANAGER.cur_task_translate(va).map(|pa| unsafe {
        (pa as *mut TaskInfo).write(TaskInfo {
            status,
            syscall_times,
            time,
        });
    });
    0
}

const PORT_CHECK: usize = 0x7;
// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    if (_port & PORT_CHECK == 0) || (_port & !PORT_CHECK != 0) {
        error!("kernel: sys_mmap Port error, port: {:#x}", _port);
        return -1;
    }
    if _start % PAGE_SIZE != 0 {
        error!("kernel: sys_mmap Start error, start: {:#x}", _start);
        return -1;
    }
    let rc = TASK_MANAGER.mmap_current_task(_start, _len, _port);
    rc
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    if _start % PAGE_SIZE != 0 {
        error!("kernel: sys_munmap Start error, start: {:08x}", _start);
        return -1;
    }
    let rc = TASK_MANAGER.munmap_current_task(_start, _len);
    rc
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
