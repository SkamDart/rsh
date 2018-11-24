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
    return_code: i32
}

impl JobOptions {
    pub fn new() -> JobOptions {
        JobOptions {
            err_fd: 2,
            flags: Vec::new(),
            in_fd: 0,
            out_fd: 1,
        }
    }
}

impl Job {
    pub fn new() -> Job {
        Job {
            cmd: String::new(),
            gid: 0,
            id: 0,
            opts: JobOptions::new(),
            pid: 0,
            status: String::new(),
        }
    }
}

impl JobResult {
    pub fn new() -> JobResult {
        JobResult {
            id: 0,
            pid: 0,
            return_code: 0,
        }
    }
}
