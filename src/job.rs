#[derive(Debug)]
pub struct Job {
    cmd: String,
    id: u32,
    opts: JobOptions,
    pid: u32,
    gid: u32,
    status: String,
}

#[derive(Debug)]
pub struct JobOptions {
    // these are raw_file descriptors
    in_fd: u32,
    out_fd: u32,
    err_fd: u32,
    flags: Vec<String>,
}

#[derive(Debug)]
pub struct JobResult {
    id: u32,
    pid: u32,
}

impl Job {
    pub fn new() {
        Job {
            in_fd: 0,
            out_fd: 1,
            err_fd: 2,
            opts: JobOptions,
            pid: None,
            gid: None,
            status: String::new(),
        }
    }
}

impl JobOptions {
    pub fn new() {
        JobOptions {
            id: None
            in_fd: 0,
            out_fd: 1,
        }
    }
}
