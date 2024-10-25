# How to do
1. Maintain the syscall_count using a array in the task_control_block.
2. using the get_time_ms() to get the current time in milliseconds.
3. get the current task from the static global variable TASK_MANNAGER.
4. get the current task status from the task_control_block.
5. move the info like status, syscall_count, time to the Taskinfo struct.
6. using unsafe code block to move the Taskinfo struct to the raw pointer pointed-memory.