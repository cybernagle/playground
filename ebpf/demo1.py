



BPF_PERF_OUTPUT(output);

struct data_t {
    int pid;
    int uid;
    chat command[16];
    chat message[12];
};

int hello(void *ctx) {
    struct data_t data = {};
    chat message[12] = "Hello World!";

    data.pid = bpf_get_current_pid_tpid() >> 32;
    data.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;

    bpf_get_current_comm(&data.command, sizeof(data.command));
    bpf_probe_read_kernel(&data.message, sizeof(data.message), message);

    output.perf_submmit(ctx, &data, sizeof(data));

    return 0;
}


BPF_PROG_ARRAY(syscall, 300);

int hello(struct bpf_raw_tracepoint_args *ctx) {
    int opcode = ctx -> args[1];
    syscall.call(ctx, opcode);

    bpf_trace_printk("Another syscall %d ", opcode);
    return 0;
}

int hello_execve(void *ctx) {
    bpf_trace_printk("Executing a program");
    return 0;
}

int hello_timer(struct bpf_raw_tracepoint_args *ctx){
    if (ctx -> args[1] == 222) {
            bpf_trace_printk("Creating a timer");
    } else if (ctx -> args[1] == 226 ){
        bpf_trace_printk("Delete a timer");
    } else {
        bpf_trace_printk("Some other timer Operation.");
    }
    return 0;
}

int ignore_opcode(void *ctx){
    return 0;
}
